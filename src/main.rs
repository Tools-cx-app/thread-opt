mod defs;
mod error;
mod framework;
mod misc;

use std::fs;

use anyhow::Result;
use log::LevelFilter;

fn init_logger() {
    let level = if fs::exists(defs::TRACING).is_ok() {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };

    #[cfg(not(target_os = "android"))]
    {
        use std::io::Write;

        let mut builder = env_logger::Builder::new();

        builder.format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        });
        builder.filter_level(level).init();
    }

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(level)
                .with_tag("thread-opt"),
        );
    }
}

fn main() -> Result<(), error::Error> {
    misc::pre_start()?;
    let config = framework::config::Config::new()?;

    init_logger();

    framework::scheduler::Sched::new().config(config).start()
}
