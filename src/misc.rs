use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::{PermissionsExt, chown},
    path::Path,
    process::Command,
};

use anyhow::Result;
use log::LevelFilter;

use crate::{defs, error};

fn init_logger() {
    let level = if fs::exists(defs::TRACING).is_ok() {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };

    use std::io::Write;

    let mut builder = env_logger::Builder::new();

    builder.format(|buf, record| {
        writeln!(
            buf,
            "[{}] [{}] {}",
            record.level(),
            record.target(),
            record.args()
        )
    });
    builder.filter_level(level).init();
}

fn lock_value<P, S>(path: P, value: S) -> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let value = value.as_ref();
    let path = path.as_ref();

    chown(path, Some(0), Some(0))?;

    let mut permissions = path.metadata()?.permissions();
    permissions.set_mode(permissions.mode() | 0o200);
    fs::set_permissions(&path, permissions)?;

    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    f.write_all(value.as_bytes())?;

    let mut permissions = path.metadata()?.permissions();
    permissions.set_mode(permissions.mode() & !0o222);
    fs::set_permissions(&path, permissions)?;
    Ok(())
}

pub fn pre_start() -> Result<(), error::Error> {
    init_logger();
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
            .filter(|s| s.contains('='))
            .map(|s| s.split_once('=').unwrap())
            .map(|(k, _)| (k.to_string(), "0".to_string()))
            .collect();
        let map: String = map.iter().map(|(k, v)| format!("{k}={v}")).collect();

        f.write_all(&map.as_bytes())?;
    }

    for p in glob::glob("/sys/devices/system/cpu/cpu*/core_ctl/m??_cpus")?.flatten() {
        lock_value(p, "9")?;
    }
    for p in glob::glob("/sys/module/migt/parameters/*cluster")?.flatten() {
        lock_value(p, "0")?;
    }

    pre_cpuset()?;

    Ok(())
}

fn pre_cpuset() -> Result<(), error::Error> {
    let cpuset = Path::new(defs::CPUSET);
    fs::create_dir_all(cpuset)?;

    let present_cpus = fs::read_to_string("/sys/devices/system/cpu/present")?;
    let mut cpus = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(cpuset.join("cpus"))?;
    cpus.write_all(present_cpus.as_bytes())?;

    let mut mems = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(cpuset.join("mems"))?;
    mems.write_all("0".as_bytes())?;
    Ok(())
}
