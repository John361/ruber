use std::fmt;
use std::io::Read;

use crate::driving::TransportTrait;
use crate::routing::RemoteCredentials;

pub enum RemoteTransport {
    Ssh(RemoteSshTransport)
}

pub struct RemoteSshTransport {
    address: String,
    credentials: RemoteCredentials,
}

impl RemoteSshTransport {
    pub fn new(address: String, credentials: RemoteCredentials) -> Self {
        Self { address, credentials }
    }
}

impl TransportTrait for RemoteSshTransport {
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

impl fmt::Display for RemoteSshTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoteTransport")
    }
}
