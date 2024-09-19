use serde::Deserialize;

use crate::routing::RemoteCredentials;

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
pub struct RemoteWatch {
    pub name: String,
    pub address: String,
    pub folder: String,
    pub credentials: RemoteCredentials,
}
