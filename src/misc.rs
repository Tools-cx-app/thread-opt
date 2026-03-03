use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
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
        let f = File::options().read(true).write(true).open(scene_cpuset)?;
        let prop = java_properties::read(BufReader::new(&f))?;
        let map: HashMap<String, String> = prop
            .into_iter()
            .filter(|(_, v)| v == "1")
            .map(|(k, _)| (k, "0".to_string()))
            .collect();

        java_properties::write(BufWriter::new(&f), &map)?;
    }

    Ok(())
}
