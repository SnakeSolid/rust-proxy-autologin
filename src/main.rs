#[macro_use]
extern crate log;

mod checker;
mod options;

use crate::checker::ProxyChecker;
use crate::options::Options;
use chrono::offset::Local;
use cron::Schedule;
use reqwest::blocking::ClientBuilder;
use reqwest::redirect::Policy;
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
        .redirect(Policy::none())
        .danger_accept_invalid_certs(true)
        .build()?;
    let checker = ProxyChecker::new(&options, &client);

    info!("Starting proxy check");

    if options.now() {
        info!("Trying to authorize immediately");

        checker.check_auth();
    }

    for datetime in schedule.upcoming(Local) {
        info!("Next check at {}", datetime);

        let now = Local::now();
        let duration = datetime - now;

        if duration.num_seconds() > 0 {
            info!("Sleep for {} seconds", duration.num_seconds());

            thread::sleep(duration.to_std()?);
        }

        checker.check_auth();
    }

    unreachable!()
}
