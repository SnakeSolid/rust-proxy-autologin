use crate::options::Options;
use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::time::Duration;

/// Checker used to check authorization status and send authorization request to proxy.
#[derive(Debug)]
pub struct ProxyChecker<'a> {
    /// URL to detect proxy status. Usually URL to some HTTP:// resource.
    detect_url: &'a str,
    /// Login used for proxy authorization
    login: &'a str,
    /// Password used for proxy authorization
    pass: &'a str,
    /// Number of retries if authorization failed.
    error_retry: usize,
    /// Interval between retries (in seconds) if authorization failed.
    error_interval: Duration,
    client: &'a Client,
}

/// Proxy check status
#[derive(Debug)]
enum CheckStatus {
    /// Status success if no authorization required by proxy.
    Success,
    /// Correct redirection to authorization page.
    Redirect(String),
    /// Response contains redirection status, but header `location` not found.
    RedirectError,
}

/// Authorization status
#[derive(Debug)]
enum LoginStatus {
    /// Authorization successfully complete.
    Success,
    /// Authorization failed, login request does not contain redirection.
    Failed,
}

impl<'a> ProxyChecker<'a> {
    /// Create new instance of checker using options and prepared `Client`:
    pub fn new<'p>(options: &'p Options, client: &'p Client) -> ProxyChecker<'p> {
        ProxyChecker {
            detect_url: options.detect_url(),
            login: options.login(),
            pass: options.pass(),
            error_retry: options.error_retry(),
            error_interval: Duration::from_secs(options.error_interval()),
            client,
        }
    }

    /// Check authorization status and login if it's necessarily. Returns `true` on success, and `false` if login
    /// failed after `error_retry` number of retries.
    pub fn check_auth(&self) -> bool {
        // Proxy check loop, repeat check 10 times on fail.
        for _ in 0..self.error_retry {
            info!("Checking proxy authorization status");

            match self.ensure_auth() {
                Ok(()) => return true,
                Err(err) => {
                    warn!("Failed to check proxy status: {}", err);
                    info!("Sleep for {} seconds", self.error_interval.as_secs());

                    thread::sleep(self.error_interval);
                }
            }
        }

        false
    }

    fn ensure_auth(&self) -> Result<(), Box<dyn Error>> {
        let status = self.detect_proxy()?;

        match status {
            CheckStatus::Success => info!("No redirection, authorization not required"),
            CheckStatus::Redirect(location) => {
                info!("Redirection found, location = {}", location);

                self.login_proxy(&location)?;
            }
            CheckStatus::RedirectError => warn!("Redirection without location field"),
        }

        Ok(())
    }

    fn login_proxy(&self, location: &str) -> Result<LoginStatus, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("login", self.login);
        params.insert("pass", self.pass);

        let response = self.client.post(location).form(&params).send()?;
        let status = response.status();

        // If login was successful proxy must redirect to original location.
        if status.is_redirection() {
            info!("Authorization complete");

            Ok(LoginStatus::Success)
        } else {
            Ok(LoginStatus::Failed)
        }
    }

    /// Try to detect proxy using redirection status and location HTTP header. If GET request to given URL returns
    /// redirection to some location (corporative proxy usually change HTTP:// protocol), then return this location
    /// as authorization page.
    fn detect_proxy(&self) -> Result<CheckStatus, Box<dyn Error>> {
        let response = self.client.get(self.detect_url).send()?;
        let status = response.status();

        if status.is_redirection() {
            if let Some(location) = response
                .headers()
                .iter()
                .filter_map(|(header, value)| {
                    if header == header::LOCATION {
                        Some(value)
                    } else {
                        None
                    }
                })
                .next()
            {
                let location = location.to_str()?;

                Ok(CheckStatus::Redirect(location.into()))
            } else {
                Ok(CheckStatus::RedirectError)
            }
        } else {
            Ok(CheckStatus::Success)
        }
    }
}
