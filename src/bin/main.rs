use ruber_lib::config::RuberConfig;
use ruber_lib::driving::Driver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logger("config/log4rs.yaml")?;
    let config = load_config("config/ruber.template.yaml")?;

    // for route in &config.routes.routes {
    //     let mut driver = Driver::new(route);
    //     driver.take_passenger("file1.txt");
    //     driver.drive()?;
    // }

    let route = &config.routes.routes[0];
    let mut driver = Driver::new(route);
    driver.take_passenger("file1.txt").await?;
    driver.drive().await?;

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
