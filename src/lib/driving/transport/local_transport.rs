use std::{fmt, io};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Context;

use crate::driving::TransportTrait;

pub struct LocalTransport {
    source: Option<PathBuf>,
    destination: Option<PathBuf>,
}

impl LocalTransport {
    pub fn new(source: Option<PathBuf>, destination: Option<PathBuf>) -> Self {
        Self { source, destination }
    }

    fn source_path(&self) -> anyhow::Result<PathBuf> {
        let path = self.source.clone().context("Cannot get source file")?;
        Ok(path)
    }

    fn destination_path(&self) -> anyhow::Result<PathBuf> {
        let path = self.destination.clone().context("Cannot get destination file")?;
        Ok(path)
    }
}

impl TransportTrait for LocalTransport {
    fn read(&self) -> anyhow::Result<Box<dyn Read>> {
        let path = self.source_path()?;
        let file = File::open(path)?;
        log::debug!("File successfully opened");

        Ok(Box::new(file))
    }

    fn write(&self, reader: &mut Box<dyn Read>) -> anyhow::Result<()> {
        let path = self.destination_path()?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        io::copy(reader, &mut file)?;
        file.flush()?;
        log::debug!("File successfully written");

        Ok(())
    }

    fn content_type(&self) -> anyhow::Result<String> {
        let path = self.source_path()?;
        let source_type = infer::get_from_path(path).context("Cannot determine content type for given source")?;

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
