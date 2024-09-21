use ruber_lib::config::RuberConfig;
use ruber_lib::agent;
use ruber_lib::error::RuberError;

#[tokio::main]
async fn main() -> Result<(), RuberError> {
    initialize_logger("config/log4rs.yaml")?;
    let config = load_config("config/ruber.template.yaml")?;

    agent::start(&config).await?;
    Ok(())
}

fn initialize_logger(config_file: &str) -> Result<(), RuberError> {
    log4rs::init_file(config_file, Default::default())?;
    log::debug!("Log4rs successfully initialized");
    Ok(())
}

fn load_config(config_file: &str) -> Result<RuberConfig, RuberError> {
    let config = RuberConfig::load(config_file)?;
    log::debug!("RuberConfig successfully loaded");
    Ok(config)
}
