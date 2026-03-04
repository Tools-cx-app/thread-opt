use anyhow::Result;

use crate::{error, framework::config::Config};

mod looper;

pub struct Sched {
    config: Option<Config>,
}

impl Sched {
    #[must_use]
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn config(mut self, c: Config) -> Self {
        self.config = Some(c);
        self
    }

    pub fn start(self) -> Result<(), error::Error> {
        let config = self
            .config
            .ok_or(error::Error::SchedulerMissing("config"))?;
        looper::Looper::new(config).enter_looper()
    }
}
