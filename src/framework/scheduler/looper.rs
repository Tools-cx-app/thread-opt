use anyhow::Result;

use crate::{
    cpuset::Cpuset,
    error,
    framework::{config::Config, thread},
};

pub struct Looper {
    config: Config,
}

impl Looper {
    pub fn new(c: Config) -> Self {
        Self { config: c }
    }

    pub fn enter_looper(&self) -> Result<(), error::Error> {
        thread::collect_pids()?;

        loop {
            for data in self.config.config()? {
                let Ok(pid) = thread::process::get_pid(data.package.clone(), data.process.clone())
                else {
                    log::trace!("{}({:?}) not find pid", data.package, data.process);
                    continue;
                };
                let appyied_pid = thread::cache::read_cache_applied();
                let mut cpuset = Cpuset::new(0, data.cpus.clone(), pid)?;

                if !appyied_pid.iter().any(|id| id == &pid) {
                    cpuset.join_tasks(pid)?;

                    thread::process::apply_cpus_to_process(pid, data.cpus.clone())?;
                }

                if !appyied_pid
                    .iter()
                    .any(|id| cpuset.tasks().iter().any(|s| s != id))
                {
                    cpuset.remove_tasks(pid);
                }
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
