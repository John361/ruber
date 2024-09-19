use std::fmt;
use std::io::Read;

use crate::driving::TransportTrait;

pub struct RemoteTransport {}

impl RemoteTransport {
    pub fn new() -> Self {
        Self {}
    }
}

impl TransportTrait for RemoteTransport {
    fn read(&self) -> anyhow::Result<Box<dyn Read>> {
        todo!()
    }

    fn write(&self, _reader: &mut Box<dyn Read>) -> anyhow::Result<()> {
        todo!()
    }

    fn content_type(&self) -> anyhow::Result<String> {
        todo!()
    }
}

impl fmt::Display for RemoteTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoteTransport")
    }
}
