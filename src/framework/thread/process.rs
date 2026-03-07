use std::{fs, mem, path::Path};

use anyhow::Result;
use glob::Pattern;

use crate::{error, framework::thread::cache::write_cache_applied};

pub fn get_process_name(pid: i32) -> Result<String, error::Error> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}

pub fn get_pid<S>(name: S, task: Option<String>) -> Result<i32, error::Error>
where
    S: ToString,
{
    let processes = procfs::process::all_processes()?;

    for process in processes {
        let process = process?;
        let cmdline = get_process_name(process.pid)?;

        if cmdline != name.to_string() {
            continue;
        }

        if let Some(task) = task.clone() {
            let tasks: Vec<_> = Path::new("/proc")
                .join(process.pid.to_string())
                .join("task")
                .read_dir()?
                .flatten()
                .filter(|s| s.file_type().unwrap().is_dir())
                .map(|s| s.file_name().to_str().unwrap().parse::<i32>().unwrap())
                .collect();

            for t in tasks {
                let comm = Path::new("/proc")
                    .join(process.pid.to_string())
                    .join("task")
                    .join(t.to_string())
                    .join("comm");
                let comm = fs::read_to_string(comm).unwrap();
                let glob_task = Pattern::new(&task)?;
                if glob_task.matches(comm.trim_matches(['\0']).trim()) {
                    return Ok(t);
                }
            }

            return Err(error::Error::PidMissing(name.to_string()));
        }
        return Ok(process.pid);
    }

    Err(error::Error::PidMissing(name.to_string()))
}

pub fn apply_cpus_to_process(pid: i32, cpus: Vec<u8>) -> Result<(), error::Error> {
    unsafe {
        let mut cpuset: libc::cpu_set_t = mem::zeroed();
        for cpu in cpus.clone() {
            libc::CPU_SET(cpu as usize, &mut cpuset);
        }

        let ret = libc::sched_setaffinity(pid, mem::size_of::<libc::cpu_set_t>(), &cpuset);

        if ret < 0 {
            return Err(error::Error::Io(std::io::Error::last_os_error()).into());
        }
    }

    log::info!(
        "apply cpu {cpus:?} to {} successful.",
        get_process_name(pid)?
    );

    write_cache_applied(pid);

    Ok(())
}
