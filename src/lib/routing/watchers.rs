use serde::Deserialize;

use crate::routing::RemoteSshCredentials;

#[derive(Debug, Deserialize)]
pub enum Watch {
    Local(LocalWatch),
    Remote(RemoteWatch),
}

#[derive(Debug, Deserialize)]
pub struct LocalWatch {
    pub name: String,
    pub folder: String,
}

#[derive(Debug, Deserialize)]
pub enum RemoteWatch {
    Ssh(RemoteSsh),
}

#[derive(Debug, Deserialize)]
pub struct RemoteSsh {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub folder: String,
    pub credentials: RemoteSshCredentials,
}
