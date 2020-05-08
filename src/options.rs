use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pgrestore-web")]
pub struct Options {
    #[structopt(
        short = "l",
        long = "login",
        name = "LOGIN",
        env = "AUTOLOGIN_LOGIN",
        help = "Proxy login"
    )]
    login: String,

    #[structopt(
        short = "p",
        long = "pass",
        name = "PASS",
        env = "AUTOLOGIN_PASS",
        hide_env_values = true,
        help = "Proxy password"
    )]
    pass: String,

    #[structopt(
        short = "d",
        long = "detect-url",
        name = "DETECT_URL",
        env = "AUTOLOGIN_DETECT_URL",
        help = "Detect proxy URL",
        default_value = "http://detectportal.firefox.com/success.txt"
    )]
    detect_url: String,

    #[structopt(
        short = "c",
        long = "cron",
        name = "CRON",
        env = "AUTOLOGIN_CRON",
        help = "Cron expression to check proxy connection",
        default_value = "0 0 6 * * mon-fri *"
    )]
    cron: String,

    #[structopt(
        short = "i",
        long = "error-interval",
        name = "INTERVAL",
        env = "AUTOLOGIN_ERROR_INTERVAL",
        help = "Interval between checks on error (seconds)",
        default_value = "300"
    )]
    error_interval: u64,

    #[structopt(
        short = "r",
        long = "error-retry",
        name = "RETRY",
        env = "AUTOLOGIN_ERROR_RETRY",
        help = "Number of checks on error",
        default_value = "3"
    )]
    error_retry: usize,
}

impl Options {
    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn pass(&self) -> &str {
        &self.pass
    }

    pub fn detect_url(&self) -> &str {
        &self.detect_url
    }

    pub fn cron(&self) -> &str {
        &self.cron
    }

    pub fn error_interval(&self) -> u64 {
        self.error_interval
    }

    pub fn error_retry(&self) -> usize {
        self.error_retry
    }
}
