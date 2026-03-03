use std::{collections::HashSet, sync::mpsc::Receiver};

use crate::framework::config::data::Data;

#[derive(Debug)]
pub struct Inner {
    rx: Receiver<HashSet<Data>>,
    data: HashSet<Data>,
}

impl Inner {
    pub fn new(rx: Receiver<HashSet<Data>>, data: HashSet<Data>) -> Self {
        Self { rx, data }
    }

    pub fn config(&mut self) -> &mut HashSet<Data> {
        if let Some(data) = self.rx.try_iter().last() {
            self.data = data;
        }

        &mut self.data
    }
}
