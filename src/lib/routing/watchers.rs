use std::path::PathBuf;

use serde::Deserialize;

use crate::routing::RemoteCredentials;

#[derive(Clone, Debug, Deserialize)]
pub enum Watch {
    Local(LocalWatch),
    Remote(RemoteWatch),
}

#[derive(Clone, Debug, Deserialize)]
pub struct LocalWatch {
    name: String,
    folder: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RemoteWatch {
    Ssh(RemoteSsh),
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoteSsh {
    name: String,
    host: String,
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
    pub fn address(&self) -> (String, u16) {
        match &self {
            RemoteWatch::Ssh(ssh) => {
                (ssh.host.clone(), ssh.port)
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
