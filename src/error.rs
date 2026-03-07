use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Procfs(#[from] procfs::ProcError),
    #[error(transparent)]
    AnyHow(#[from] anyhow::Error),
    #[error("Missing {0} when building Scheduler")]
    SchedulerMissing(&'static str),
    #[error("Missing {0} when parseing config")]
    ConfigMissing(&'static str),
    #[error("Missing pid when getting {0}")]
    PidMissing(String),
    #[error(transparent)]
    Glob(#[from] glob::PatternError),
}
