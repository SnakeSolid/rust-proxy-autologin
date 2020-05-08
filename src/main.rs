#[macro_use]
extern crate log;

mod checker;
mod options;

use crate::checker::ProxyChecker;
use crate::options::Options;
use chrono::offset::Local;
use cron::Schedule;
use reqwest::blocking::ClientBuilder;
use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let options = Options::from_args();
    let schedule = Schedule::from_str(options.cron())?;
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true)
        .build()?;
    let checker = ProxyChecker::new(&options, &client);

    info!("Starting proxy check");

    for datetime in schedule.upcoming(Local) {
        info!("Next check at {}", datetime);

        let now = Local::now();
        let duration = datetime - now;

        if duration.num_seconds() > 0 {
            info!("Sleep for {} seconds", duration.num_seconds());

            thread::sleep(duration.to_std()?);
        }

        // Proxy check loop, repeat check 10 times on fail.
        for _ in 0..options.error_retry() {
            info!("Checking proxy authorization status");

            match checker.ensure_auth() {
                Ok(()) => break,
                Err(err) => {
                    warn!("Failed to check proxy status: {}", err);
                    info!("Sleep for {} seconds", options.error_interval());

                    thread::sleep(Duration::from_secs(options.error_interval()));
                }
            }
        }
    }

    unreachable!()
}
