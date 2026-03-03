pub mod data;
mod parse;

use std::collections::HashSet;

use anyhow::Result;

use crate::{defs, framework::config::data::Data};

pub fn parse_prop() -> Result<HashSet<Data>> {
    log::debug!("Starting parse config");

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

    log::debug!("parse config was done");
    Ok(map)
}
