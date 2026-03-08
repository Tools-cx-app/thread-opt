pub mod data;
mod parse;

use std::{
    collections::HashSet,
    path::Path,
    sync::{LazyLock, Mutex},
};

use anyhow::Result;
use inotify::{Inotify, WatchMask};

use crate::{defs, framework::config::data::Data};

static PROP: LazyLock<Mutex<HashSet<Data>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

#[derive(Clone)]
pub struct Config;

impl Config {
    pub fn new() -> Result<Self> {
        std::thread::Builder::new()
            .name("ConfigWatcher".to_string())
            .spawn(|| {
                log::debug!("Config Watcher thread starting");
                parser().unwrap_or_else(|e| log::error!("{e:#?}"));
                panic!("An unrecoverable error occurred!");
            })?;

        Ok(Self)
    }

    pub fn config(&self) -> Result<HashSet<Data>> {
        let prop = PROP.lock().unwrap().clone();

        log::trace!("prop: {prop:?}");

        Ok(prop)
    }
}

fn parser() -> Result<()> {
    loop {
        let prop = parse::parse_prop(defs::CONFIG_PATH)?;
        let mut map = HashSet::new();

        for (k, v) in prop {
            let mut data = Data::default();

            if k.contains('{') || k.contains('}') {
                let (process, package) = parse::parse_process(k.clone())?;

                data.process = Some(process);
                data.package = package;
            } else {
                data.package = k;
            }

            data.cpus = parse::parse_cpus(v.to_string());

            map.insert(data);
        }

        {
            let mut locker = PROP.lock().unwrap();
            locker.extend(map);
        }

        wait_until_update(defs::CONFIG_PATH)?;
    }
}

fn wait_until_update<P>(p: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut inotify = Inotify::init()?;

    inotify
        .watches()
        .add(p.as_ref(), WatchMask::MODIFY | WatchMask::CLOSE_WRITE)?;

    let mut buffer = [0; 1024];
    inotify.read_events_blocking(&mut buffer)?;

    Ok(())
}
