use serde::Deserialize;

use crate::routing::Watch;

#[derive(Debug, Deserialize)]
pub struct Route {
    pub source: Watch,
    pub destination: Vec<Watch>,
}
