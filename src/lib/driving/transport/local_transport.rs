use std::fmt;

use crate::driving::TransportTrait;

pub struct LocalTransport {}

impl LocalTransport {
    pub fn new() -> Self {
        Self {}
    }
}

impl TransportTrait for LocalTransport {
    fn drive_with(&self) {
        todo!()
    }
}

impl fmt::Display for LocalTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocalTransport")
    }
}
