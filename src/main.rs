mod defs;
mod framework;
mod misc;

use anyhow::Result;

fn init_logger() {
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
        builder.filter_level(log::LevelFilter::Debug).init();
    }

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Debug)
                .with_tag("thread-opt"),
        );
    }
}

fn main() -> Result<()> {
    misc::pre_start()?;
    let config = framework::config::parse_prop()?;

    init_logger();

    framework::scheduler::Sched::new().config(config).start()
}
