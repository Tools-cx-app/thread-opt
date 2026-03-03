use anyhow::Result;

use crate::framework::{config::Config, thread};

pub struct Looper {
    config: Config,
}

impl Looper {
    pub fn new(c: Config) -> Self {
        Self { config: c }
    }

    pub fn enter_looper(&self) -> Result<()> {
        thread::collect_pids()?;

        loop {
            for data in self.config.config()? {
                let pid = thread::process::get_pid(data.package.clone(), data.process.clone())?;
                let appyied_pid = thread::cache::read_cache_applied();

                if !appyied_pid.iter().any(|id| id == &pid) {
                    thread::process::apply_cpus_to_process(pid, data.cpus.clone())?;
                }
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
