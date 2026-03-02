use std::collections::HashSet;

use anyhow::Result;

use crate::framework::config::data::Data;

mod looper;

pub struct Sched {
    config: HashSet<Data>,
}

impl Sched {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HashSet::new(),
        }
    }

    pub fn config(&mut self, c: HashSet<Data>) -> &mut Self {
        self.config = c;
        self
    }

    pub fn start(&self) -> Result<()> {
        looper::Looper::new(self.config.clone()).enter_looper()
    }
}
