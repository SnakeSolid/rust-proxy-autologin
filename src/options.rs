use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pgrestore-web")]
pub struct Options {
    #[structopt(short, long, env = "AUTOLOGIN_LOGIN", help = "Proxy login")]
    login: String,

    #[structopt(
        short,
        long,
        env = "AUTOLOGIN_PASS",
        hide_env_values = true,
        help = "Proxy password"
    )]
    pass: String,

    #[structopt(short, long, help = "Authorize immediately after start")]
    now: bool,

    #[structopt(
        short,
        long,
        name = "url",
        env = "AUTOLOGIN_DETECT_URL",
        help = "Detect proxy URL",
        default_value = "http://detectportal.firefox.com/success.txt"
    )]
    detect_url: String,

    #[structopt(
        short,
        long,
        env = "AUTOLOGIN_CRON",
        help = "Cron expression to check proxy connection",
        default_value = "0 0 6 * * mon-fri *"
    )]
    cron: String,

    #[structopt(
        short = "i",
        long,
        name = "interval",
        env = "AUTOLOGIN_ERROR_INTERVAL",
        help = "Interval between checks on error (seconds)",
        default_value = "300"
    )]
    error_interval: u64,

    #[structopt(
        short = "r",
        long,
        name = "reties",
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

    pub fn now(&self) -> bool {
        self.now
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
