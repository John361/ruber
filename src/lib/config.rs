use serde::Deserialize;

use crate::error::RuberError;
use crate::routing::Routes;

#[derive(Debug, Deserialize)]
pub struct RuberConfig {
    pub routes: Routes,
}

impl RuberConfig {
    pub fn load(config_file: &str) -> Result<Self, RuberError> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(config_file))
            .build()
            .map_err(|e| {
                let error = RuberError::Config(e);
                log::error!("RuberError: {:?}", error);
                error
            })?;

        let crust_config = config
            .try_deserialize::<RuberConfig>()
            .map_err(|e| {
                let error = RuberError::Config(e);
                log::error!("RuberError: {:?}", error);
                error
            })?;

        Ok(crust_config)
    }
}
