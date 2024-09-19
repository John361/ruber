use std::path::PathBuf;

use serde::Deserialize;

use crate::routing::RemoteCredentials;

#[derive(Debug, Deserialize)]
pub enum Watch {
    Local(LocalWatch),
    Remote(RemoteWatch),
}

#[derive(Debug, Deserialize)]
pub struct LocalWatch {
    name: String,
    folder: String,
}

#[derive(Debug, Deserialize)]
pub enum RemoteWatch {
    Ssh(RemoteSsh),
}

#[derive(Debug, Deserialize)]
pub struct RemoteSsh {
    name: String,
    address: String,
    port: u16,
    folder: String,
    credentials: RemoteCredentials,
}

impl Watch {
    pub fn name(&self) -> &str {
        match &self {
            Watch::Local(local) => {
                &local.name
            }

            Watch::Remote(remote) => {
                match remote {
                    RemoteWatch::Ssh(ssh) => {
                        &ssh.name
                    }
                }
            }
        }
    }

    pub fn folder(&self) -> PathBuf {
        match &self {
            Watch::Local(local) => {
                PathBuf::from(&local.folder)
            }

            Watch::Remote(remote) => {
                match remote {
                    RemoteWatch::Ssh(ssh) => {
                        PathBuf::from(&ssh.folder)
                    }
                }
            }
        }
    }
}

impl RemoteWatch {
    pub fn address(&self) -> String {
        match &self {
            RemoteWatch::Ssh(ssh) => {
                format!("{}:{}", ssh.address, ssh.port)
            }
        }
    }

    pub fn credentials(&self) -> RemoteCredentials {
        match &self {
            RemoteWatch::Ssh(ssh) => {
                ssh.credentials.clone()
            }
        }
    }
}
