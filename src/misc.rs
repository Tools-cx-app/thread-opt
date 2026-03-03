use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    process::Command,
};

use anyhow::Result;

use crate::defs;

pub fn pre_start() -> Result<()> {
    std::panic::set_hook(Box::new(|p| {
        log::error!("panic info: {}", p);
    }));

    let processes = procfs::process::all_processes()?;
    let scene_cpuset = Path::new(defs::SCENE_CPUSET);

    for process in processes.flatten() {
        let cmdline = process.cmdline()?;

        if cmdline.iter().any(|s| s == "AppOpt" || s == "AsoulOpt") {
            let _ = Command::new("kill")
                .args(["-9", process.pid.to_string().as_str()])
                .output();
        }
    }

    if scene_cpuset.exists() {
        let mut f = File::options().read(true).write(true).open(scene_cpuset)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        
        let map: HashMap<String, String> = buf
            .lines()
            .map(|s| s.split_once('=').unwrap())
            .map(|(k, _)| (k.to_string(), "0".to_string()))
            .collect();
        let map: String = map.iter().map(|(k, v)| format!("{k}={v}")).collect();

        f.write_all(&map.as_bytes())?;
    }

    Ok(())
}
