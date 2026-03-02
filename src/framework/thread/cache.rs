use std::sync::{Mutex, RwLock};

pub(super) static PID: RwLock<Mutex<Vec<i32>>> = RwLock::new(Mutex::new(Vec::new()));
pub(super) static APPLIED_PID: RwLock<Mutex<Vec<i32>>> = RwLock::new(Mutex::new(Vec::new()));

pub fn write_cache_applied(pid: i32) {
    APPLIED_PID.write().unwrap().lock().unwrap().push(pid);
}

pub fn read_cache_applied() -> Vec<i32> {
    APPLIED_PID.read().unwrap().lock().unwrap().clone()
}

pub fn write_cache(pid: i32) {
    PID.write().unwrap().lock().unwrap().push(pid);
}

pub fn read_cache() -> Vec<i32> {
    PID.read().unwrap().lock().unwrap().clone()
}
