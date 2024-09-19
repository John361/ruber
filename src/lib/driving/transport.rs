mod local_transport;
mod remote_transport;

pub use local_transport::*;
pub use remote_transport::*;

use std::fmt;
use std::io::Read;

pub trait TransportTrait: fmt::Display {
    fn read(&self) -> anyhow::Result<Box<dyn Read>>;
    fn write(&self, reader: &mut Box<dyn Read>) -> anyhow::Result<()>;
    fn content_type(&self) -> anyhow::Result<String>;
}
