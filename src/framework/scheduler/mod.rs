use anyhow::Result;

use crate::framework::config::Config;

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

    pub fn start(self) -> Result<()> {
        looper::Looper::new(self.config.unwrap().clone()).enter_looper()
    }
}
