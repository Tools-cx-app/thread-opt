pub mod data;
mod inner;
mod parse;

use std::{collections::HashSet, sync::mpsc, thread};

use anyhow::Result;

use crate::{
    defs,
    framework::config::{data::Data, inner::Inner},
};

#[derive(Debug)]
pub struct Config {
    inner: Inner,
}

impl Config {
    pub fn new() -> Result<Self> {
        let (sx, rx) = mpsc::channel();
        let prop = parse_prop()?;
        let inner = Inner::new(rx, prop);

        thread::Builder::new()
            .name("ConfigThread".into())
            .spawn(move || {
                parse::wait_and_read(defs::CONFIG_PATH, &sx)
                    .unwrap_or_else(|e| log::error!("{e:#?}"));
                panic!("An unrecoverable error occurred!");
            })?;

        Ok(Self { inner })
    }

    pub fn data(&mut self) -> &mut HashSet<Data> {
        self.inner.config()
    }
}

fn parse_prop() -> Result<HashSet<Data>> {
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
