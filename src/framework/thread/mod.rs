pub mod cache;
mod error;
pub mod process;

use anyhow::Result;

pub fn collect_pids() -> Result<()> {
    std::thread::Builder::new()
        .name("Pids-Collect".to_string())
        .spawn(|| {
            collect_pid().unwrap();
        })?;

    Ok(())
}

fn collect_pid() -> Result<()> {
    loop {
        let processes = procfs::process::all_processes()?;
        let cache_applied_pids = cache::APPLIED_PID.write().unwrap();
        let cache_pids = cache::PID.write().unwrap();

        let mut pids = Vec::new();

        for process in processes.flatten() {
            if cache_applied_pids
                .lock()
                .unwrap()
                .iter()
                .any(|s| s != &process.pid)
            {
                let pos = cache_applied_pids
                    .lock()
                    .unwrap()
                    .iter()
                    .position(|x| x == &process.pid)
                    .unwrap();
                cache_applied_pids.lock().unwrap().remove(pos);
            }

            pids.push(process.pid);
        }

        cache_pids.lock().unwrap().clear();
        cache_pids.lock().unwrap().extend(pids);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
