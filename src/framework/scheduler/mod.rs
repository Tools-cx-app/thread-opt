
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
        let config = self.config.unwrap();
        looper::Looper::new(config).enter_looper()
    }
}
