use std::{fs, mem, path::Path};

use anyhow::Result;

use crate::framework::thread::{cache::write_cache_applied, error};

pub fn get_process_name(pid: i32) -> Result<String> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}

pub fn get_pid<S>(name: S, task: Option<String>) -> Result<i32>
where
    S: ToString,
{
    let processes = procfs::process::all_processes()?;

    for process in processes {
        let process = process?;

        if let Some(task) = task.clone() {
            let tasks = process.tasks()?;

            for t in tasks {
                let t = t?;
                let comm = Path::new("/proc")
                    .join(process.pid.to_string())
                    .join("task")
                    .join(t.pid.to_string())
                    .join("comm");
                let comm = fs::read_to_string(comm).unwrap();
                if comm.trim_matches(['\0']).trim() == task {
                    return Ok(t.pid);
                }
            }

            return Err(error::Error::Pid.into());
        }

        let cmdline = get_process_name(process.pid)?;
        if cmdline == name.to_string() {
            return Ok(process.pid);
        }
    }

    Err(error::Error::Pid.into())
}

pub fn apply_cpus_to_process(pid: i32, cpus: Vec<u8>) -> Result<()> {
    unsafe {
        let mut cpuset: libc::cpu_set_t = mem::zeroed();
        for cpu in cpus.clone() {
            libc::CPU_SET(cpu as usize, &mut cpuset);
        }

        let ret = libc::sched_setaffinity(pid, mem::size_of::<libc::cpu_set_t>(), &cpuset);

        if ret < 0 {
            return Err(error::Error::SchedSetaffinity(std::io::Error::last_os_error()).into());
        }
    }

    log::info!(
        "apply cpu {cpus:?} to {} successful.",
        get_process_name(pid)?
    );

    write_cache_applied(pid);

    Ok(())
}
