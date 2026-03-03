use std::sync::RwLock;

pub(super) static PID: RwLock<Vec<i32>> = RwLock::new(Vec::new());
pub(super) static APPLIED_PID: RwLock<Vec<i32>> = RwLock::new(Vec::new());

pub fn write_cache_applied(pid: i32) {
    APPLIED_PID.write().unwrap().push(pid);
}

pub fn read_cache_applied() -> Vec<i32> {
    APPLIED_PID.read().unwrap().clone()
}
