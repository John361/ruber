use std::fmt;

use crate::driving::TransportTrait;

pub struct RemoteTransport {}

impl RemoteTransport {
    pub fn new() -> Self {
        Self {}
    }
}

impl TransportTrait for RemoteTransport {
    fn drive_with(&self) {
        todo!()
    }
}

impl fmt::Display for RemoteTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoteTransport")
    }
}
