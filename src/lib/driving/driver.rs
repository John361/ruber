use std::path::PathBuf;

use crate::driving::{LocalTransport, RemoteTransport, TransportTrait};
use crate::routing::{Route, Watch};

pub struct Driver<'a> {
    route: &'a Route,
    passenger: Option<PathBuf>,
    transport_a: Option<Box<dyn TransportTrait>>,
    transport_b: Option<Box<dyn TransportTrait>>,
}

impl<'a> Driver<'a> {
    pub fn new(route: &'a Route) -> Self {
        Self { route, passenger: None, transport_a: None, transport_b: None }
    }

    pub fn take_passenger(&mut self, file_name: &str) {
        self.take_appropriate_transport_to_passenger();
        self.passenger = Some(self.route.source.folder().join(file_name));
        log::info!("Passenger {} successfully taken in charge with {} transport",
            self.passenger.as_ref().unwrap().display(),
            self.transport_a.as_ref().unwrap()
        );
    }

    fn take_appropriate_transport_to_passenger(&mut self) {
        match self.route.source {
            Watch::Local(_) => {
                self.transport_a = Some(Box::new(LocalTransport::new()))
            }

            Watch::Remote(_) => {
                self.transport_a = Some(Box::new(RemoteTransport::new()))
            }
        }
    }

    fn take_appropriate_transport_to_destination(&mut self, destination: &Watch) {
        match destination {
            Watch::Local(_) => {
                self.transport_b = Some(Box::new(LocalTransport::new()))
            }

            Watch::Remote(_) => {
                self.transport_b = Some(Box::new(RemoteTransport::new()))
            }
        }
    }

    pub fn drive(&mut self) -> anyhow::Result<()> {
        for destination in &self.route.destinations {
            self.take_appropriate_transport_to_destination(destination);
            log::info!("Driving passenger from {} to {}...",
                self.passenger.as_ref().unwrap().display(),
                destination.folder().display()
            );
            log::info!("Passenger successfully driven to destination");
        }

        Ok(())
    }
}
