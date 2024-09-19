use std::fmt;

pub trait TransportTrait: fmt::Display {
    fn drive_with(&self);
}

pub struct LocalTransport {}

pub struct RemoteTransport {}

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
