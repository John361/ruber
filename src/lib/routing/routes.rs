use serde::Deserialize;

use crate::routing::Watch;

#[derive(Clone, Debug, Deserialize)]
pub struct Routes {
    pub routes: Vec<Route>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Route {
    pub name: String,
    pub source: Watch,
    pub destinations: Vec<Watch>,
}
