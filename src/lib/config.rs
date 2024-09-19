use anyhow::Context;
use serde::Deserialize;

use crate::routing::Routes;

#[derive(Debug, Deserialize)]
pub struct RuberConfig {
    pub routes: Routes,
}

impl RuberConfig {
    pub fn load(config_file: &str) -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(config_file))
            .build()
            .context(format!("Failed to build config from {0}", config_file))?;

        let crust_config = config
            .try_deserialize::<RuberConfig>()
            .context("Failed to deserialize into RuberConfig")?;

        Ok(crust_config)
    }
}
