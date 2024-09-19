use std::path::PathBuf;

use crate::routing::Route;

pub struct Driver<'a> { // TODO: add cars to drive
    route: &'a Route,
    passenger: Option<PathBuf>,
}

impl<'a> Driver<'a> {
    pub fn new(route: &'a Route) -> Self {
        Self { route, passenger: None }
    }

    pub fn take(&mut self, file_name: &str) {
        self.passenger = Some(self.route.source.folder().join(file_name));
        log::info!("Passenger successfully taken in charge: {}", self.passenger.as_ref().unwrap().display());
    }

    pub fn drive(&self) -> anyhow::Result<()> {
        for destination in &self.route.destinations {
            log::info!("Driving passenger from {} to {}...",
                self.passenger.as_ref().unwrap().display(),
                destination.folder().display()
            );
            log::info!("Passenger successfully driven to destination");
        }

        Ok(())
    }
}
