mod cpuset;
mod defs;
mod error;
mod framework;
mod misc;

use anyhow::Result;

fn main() -> Result<(), error::Error> {
    misc::pre_start()?;
    let config = framework::config::Config::new()?;

    framework::scheduler::Sched::new().config(config).start()
}
