use std::collections::HashSet;

use anyhow::Result;

use crate::framework::{config::data::Data, thread};

pub struct Looper {
    config: HashSet<Data>,
}

impl Looper {
    pub fn new(c: HashSet<Data>) -> Self {
        Self { config: c }
    }

    pub fn enter_looper(&self) -> Result<()> {
        thread::collect_pids()?;

        loop {
            for data in self.config.clone() {
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
