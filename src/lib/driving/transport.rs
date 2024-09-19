mod local_transport;
mod remote_transport;

pub use local_transport::*;
pub use remote_transport::*;

use std::fmt;

pub trait TransportTrait: fmt::Display {
    fn drive_with(&self);
}
