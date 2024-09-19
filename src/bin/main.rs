#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logger("config/log4rs.yaml")?;

    Ok(())
}

fn initialize_logger(config_file: &str) -> anyhow::Result<()> {
    log4rs::init_file(config_file, Default::default())?;
    log::debug!("Log4rs successfully initialized");
    Ok(())
}
