use std::{collections::HashSet, fs, path::Path, sync::mpsc::Sender};

use anyhow::{Result, anyhow};
use inotify::{Inotify, WatchMask};

use crate::framework::config::data::Data;

pub fn wait_and_read<P>(p: P, sx: &Sender<HashSet<Data>>) -> Result<()>
where
    P: AsRef<Path>,
{
    loop {
        let prop = parse_prop(p.as_ref())?;
        let mut map = HashSet::new();

        for (k, v) in prop {
            let mut data = Data::default();

            if k.contains('{') || k.contains('}') {
                let (process, package) = parse_process(k.clone())?;

                data.process = Some(process);
                data.package = package;
            } else {
                data.package = k;
            }

            data.cpus = parse_cpus(v.to_string());

            map.insert(data);
        }
        let _ = sx.send(map);

        wait_until_update(p.as_ref())?;
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

pub fn parse_prop<P>(p: P) -> Result<HashSet<(String, String)>>
where
    P: AsRef<Path>,
{
    let f = fs::read_to_string(p.as_ref())?;
    let map: HashSet<(String, String)> = f
        .lines()
        .filter(|s| s.contains('='))
        .map(|s| s.split_once('=').unwrap())
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();

    Ok(map)
}

pub fn parse_process<S>(k: S) -> Result<(String, String)>
where
    S: ToString,
{
    let k = k.to_string();

    let Some(pos_head) = k.find('{') else {
        return Err(anyhow!("Missing character '{'".to_string()));
    };
    let Some(pos_end) = k.find('}') else {
        return Err(anyhow!("Missing character '{'".to_string()));
    };
    let process = k.get(pos_head + 1..pos_end).unwrap();
    let package = k.get(..pos_head).unwrap();

    Ok((process.to_string(), package.to_string()))
}

pub fn parse_cpus<S>(v: S) -> Vec<u8>
where
    S: ToString,
{
    let v = v.to_string();
    if v.contains('-') {
        let pos: Vec<&str> = v.split('-').collect();

        let pos_start: u8 = pos[0].parse().unwrap();
        let pos_end: u8 = pos[1].parse().unwrap();
        (pos_start..pos_end).collect()
    } else {
        vec![v.parse::<u8>().unwrap()]
    }
}
