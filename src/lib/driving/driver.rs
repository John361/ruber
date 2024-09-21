use std::path::PathBuf;
use anyhow::Context;

use crate::driving::{LocalTransport, RemoteSshTransport, TransportTrait};
use crate::error::DrivingError;
use crate::routing::{RemoteWatch, Route, Watch};

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

    pub async fn take_passenger(&mut self, file_name: &str) -> Result<(), DrivingError> {
        self.passenger = Some(PathBuf::from(file_name));
        self.take_appropriate_transport_to_passenger().await?;
        log::info!("Passenger {} successfully taken in charge with {} transport",
            self.passenger.as_ref().unwrap().display(),
            self.transport_a.as_ref().unwrap()
        );

        Ok(())
    }

    async fn take_appropriate_transport_to_passenger(&mut self) -> Result<(), DrivingError> {
        match &self.route.source {
            Watch::Local(_) => {
                let mut source = self.route.source.folder();
                source.push(self.passenger.clone().unwrap());
                self.transport_a = Some(Box::new(LocalTransport::new(
                    Some(source),
                    None
                )));
            }

            Watch::Remote(remote) => {
                match remote {
                    RemoteWatch::Ssh(_) => {
                        let credentials = remote.credentials();
                        let transport = RemoteSshTransport::new(remote.address(), credentials).await?;
                        self.transport_a = Some(Box::new(transport));
                    }
                }
            }
        }

        Ok(())
    }

    async fn take_appropriate_transport_to_destination(&mut self, destination: &Watch) -> Result<(), DrivingError> {
        match destination {
            Watch::Local(_) => {
                let mut source = destination.folder();
                source.push(self.passenger.clone().unwrap());
                self.transport_b = Some(Box::new(LocalTransport::new(
                    None,
                    Some(source)
                )));
            }

            Watch::Remote(remote) => {
                match remote {
                    RemoteWatch::Ssh(_) => {
                        let credentials = remote.credentials().clone();
                        let transport = RemoteSshTransport::new(remote.address(), credentials).await?;
                        self.transport_b = Some(Box::new(transport));
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn drive(&mut self) -> Result<(), DrivingError> {
        for destination in &self.route.destinations {
            self.take_appropriate_transport_to_destination(destination).await?;
            log::info!("Driving passenger from {} to {}...",
                self.route.source.folder().display(),
                destination.folder().display()
            );

            let transport_a = self.transport_a.take()
                .context("Cannot take transport a")
                .map_err(|e| {
                    let error = DrivingError::Unknown(e);
                    log::error!("DrivingError: {:?}", error);
                    error
                })?;

            let transport_b = self.transport_b.take()
                .context("Cannot take transport b")
                .map_err(|e| {
                    let error = DrivingError::Unknown(e);
                    log::error!("DrivingError: {:?}", error);
                    error
                })?;

            let mut content = transport_a.read()?;
            transport_b.write(&mut content)?;

            log::info!("Passenger successfully driven to destination");
        }

        Ok(())
    }
}
