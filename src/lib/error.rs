use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuberError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AgentError {
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
