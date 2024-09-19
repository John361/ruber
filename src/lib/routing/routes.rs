use serde::Deserialize;

use crate::routing::Watch;

#[derive(Debug, Deserialize)]
pub struct Routes {
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub source: Watch,
    pub destination: Vec<Watch>,
}
