use std::fmt;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Data {
    pub package: String,
    pub process: Option<String>,
    pub cpus: Vec<u8>,
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Data")
            .field("package", &self.package)
            .field("need_bind_process", &self.process)
            .field("cpus", &self.cpus)
            .finish()
    }
}
