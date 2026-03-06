use std::{collections::HashSet, fs, path::Path};

use anyhow::Result;

use crate::error;

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

pub fn parse_process<S>(k: S) -> Result<(String, String), error::Error>
where
    S: ToString,
{
    let k = k.to_string();

    let pos_head = k.find('{').ok_or(error::Error::ConfigMissing("{"))?;
    let pos_end = k.find('}').ok_or(error::Error::ConfigMissing("}"))?;
    let process = k.get(pos_head + 1..pos_end).unwrap();
    let package = k.get(..pos_head).unwrap();

    Ok((process.to_string(), package.to_string()))
}

pub fn parse_cpus<S>(v: S) -> Vec<u8>
where
    S: ToString,
{
    let v = v.to_string();
    let mut cpus = Vec::new();

    if v.contains('-') {
        let pos: Vec<&str> = v.split('-').collect();

        let pos_start: u8 = pos[0].parse().unwrap();
        let pos_end: u8 = pos[1].parse().unwrap();
        cpus = (pos_start..pos_end).collect()
    }
    if v.contains(',') {
        let pos: Vec<&str> = v.split(',').collect();

        cpus.extend(pos.iter().map(|s| s.parse::<u8>().unwrap()));
    }

    if cpus.is_empty() {
        cpus = vec![v.parse::<u8>().unwrap()];
    }

    cpus
}
