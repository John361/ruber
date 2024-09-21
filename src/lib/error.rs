use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuberError {
    #[error("Agent error")]
    Agent(#[from] AgentError),

    #[error("Config error")]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Notify error")]
    Notify(#[from] notify::Error),

    #[error("Driving error")]
    Driving(#[from] DrivingError),

    #[error("Join error")]
    Join(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum DrivingError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Routing error")]
    Routing(#[from] RoutingError),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("Bad address")]
    Address(#[from] std::net::AddrParseError),

    #[error("Ssh error")]
    Ssh(#[from] async_ssh2_lite::Error),
}
