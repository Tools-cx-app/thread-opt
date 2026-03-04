pub mod cache;
pub mod process;

use anyhow::Result;

pub fn collect_pids() -> Result<()> {
    std::thread::Builder::new()
        .name("Pids-Collect".to_string())
        .spawn(|| {
            log::debug!("Pids-Collect thread starting");
            collect_pid().unwrap();
        })?;

    Ok(())
}

fn collect_pid() -> Result<()> {
    loop {
        let processes = procfs::process::all_processes()?;
        let mut cache_applied_pids = cache::APPLIED_PID.write().unwrap();
        let mut cache_pids = cache::PID.write().unwrap();

        let mut pids = Vec::new();

        for process in processes.flatten() {
            if cache_applied_pids.iter().any(|s| s != &process.pid) {
                let Some(pos) = cache_applied_pids.iter().position(|x| x == &process.pid) else {
                    continue;
                };
                cache_applied_pids.remove(pos);
            }

            pids.push(process.pid);
        }

        cache_pids.clear();
        cache_pids.extend(pids);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
