use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to sched_setaffinity: {0}")]
    SchedSetaffinity(#[from] std::io::Error),
    #[error("Failed to get pid")]
    Pid,
    #[error("Failed to write pid cache")]
    PidCache(#[from] procfs::ProcError),
}
