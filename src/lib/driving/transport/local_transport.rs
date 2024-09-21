use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fmt, io};

use anyhow::Context;

use crate::driving::TransportTrait;
use crate::error::DrivingError;

pub struct LocalTransport {
    source: Option<PathBuf>,
    destination: Option<PathBuf>,
}

impl LocalTransport {
    pub fn new(source: Option<PathBuf>, destination: Option<PathBuf>) -> Self {
        Self {
            source,
            destination,
        }
    }

    fn source_path(&self) -> Result<PathBuf, DrivingError> {
        let path = self
            .source
            .clone()
            .context("Cannot get source file")
            .map_err(|e| {
                let error = DrivingError::Unknown(e);
                log::error!("DrivingError: {:?}", error);
                error
            })?;

        Ok(path)
    }

    fn destination_path(&self) -> Result<PathBuf, DrivingError> {
        let path = self
            .destination
            .clone()
            .context("Cannot get destination file")
            .map_err(|e| {
                let error = DrivingError::Unknown(e);
                log::error!("DrivingError: {:?}", error);
                error
            })?;

        Ok(path)
    }
}

impl TransportTrait for LocalTransport {
    fn read(&self) -> Result<Box<dyn Read>, DrivingError> {
        let path = self.source_path()?;
        let file = File::open(path).map_err(|e| {
            let error = DrivingError::Io(e);
            log::error!("DrivingError: {:?}", error);
            error
        })?;

        log::debug!("File successfully opened");
        Ok(Box::new(file))
    }

    fn write(&self, reader: &mut Box<dyn Read>) -> Result<(), DrivingError> {
        let path = self.destination_path()?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| {
                let error = DrivingError::Io(e);
                log::error!("DrivingError: {:?}", error);
                error
            })?;

        io::copy(reader, &mut file).map_err(|e| {
            let error = DrivingError::Io(e);
            log::error!("DrivingError: {:?}", error);
            error
        })?;

        file.flush().map_err(|e| {
            let error = DrivingError::Io(e);
            log::error!("DrivingError: {:?}", error);
            error
        })?;

        log::debug!("File successfully written");
        Ok(())
    }

    fn content_type(&self) -> Result<String, DrivingError> {
        let path = self.source_path()?;
        let source_type = infer::get_from_path(path).map_err(|e| {
            let error = DrivingError::Io(e);
            log::error!("DrivingError: {:?}", error);
            error
        })?;

        if source_type.is_some() {
            let mime_type = source_type.unwrap().mime_type().to_string();
            log::debug!("Mime-type detected: {}", &mime_type);
            Ok(mime_type)
        } else {
            log::warn!("No mime-type detected");
            Ok("".to_string())
        }
    }
}

impl fmt::Display for LocalTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocalTransport")
    }
}
