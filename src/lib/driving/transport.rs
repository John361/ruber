mod local_transport;
mod remote_transport;

pub use local_transport::*;
pub use remote_transport::*;

use std::fmt;
use std::io::Read;

use crate::error::DrivingError;

pub trait TransportTrait: fmt::Display + Send {
    fn read(&self) -> Result<Box<dyn Read>, DrivingError>;
    fn write(&self, reader: &mut Box<dyn Read>) -> Result<(), DrivingError>;
    fn content_type(&self) -> Result<String, DrivingError>;
}
