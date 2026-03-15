use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use bytemuck::cast_slice;

use crate::{defs, error};

pub struct Cpuset {
    mums: i32,
    cpus: Vec<u8>,
    path: PathBuf,
    tasks: Vec<i32>,
}

impl Cpuset {
    pub fn new(mums: i32, cpus: Vec<u8>, tid: i32) -> Result<Self, error::Error> {
        let path = Path::new(defs::CPUSET).join(tid.to_string());

        let _ = fs::create_dir_all(&path)?;
        let str_cpus: Vec<String> = cpus.iter().map(|s| s.to_string()).collect();

        fs::write(path.join("cpus"), str_cpus.join(","))?;
        Ok(Self {
            mums,
            cpus,
            path,
            tasks: Vec::new(),
        })
    }

    pub fn join_tasks(&mut self, tid: i32) -> Result<(), error::Error> {
        self.tasks.push(tid);

        let tasks = self.path.join("tasks");

        let mut f = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(tasks)?;

        let tasks: &[u8] = cast_slice(&self.tasks);
        f.write_all(tasks)?;
        Ok(())
    }
}
