mod defs;
mod framework;

use anyhow::Result;

fn main() -> Result<()> {
    let config = framework::config::parse_prop()?;

    framework::scheduler::Sched::new().config(config).start()
}
