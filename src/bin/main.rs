use ruber_lib::config::RuberConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logger("config/log4rs.yaml")?;

    let config = load_config("config/ruber.template.yaml");
    println!("{:#?}", config);

    Ok(())
}

fn initialize_logger(config_file: &str) -> anyhow::Result<()> {
    log4rs::init_file(config_file, Default::default())?;
    log::debug!("Log4rs successfully initialized");
    Ok(())
}

fn load_config(config_file: &str) -> anyhow::Result<RuberConfig> {
    let config = RuberConfig::load(config_file)?;
    log::debug!("RuberConfig successfully loaded");
    Ok(config)
}
