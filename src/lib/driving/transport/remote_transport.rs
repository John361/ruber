use std::fmt;
use std::io::Read;
use std::net::SocketAddr;
use std::path::PathBuf;

use async_ssh2_lite::AsyncSession;
use tokio::net::TcpStream;

use crate::driving::TransportTrait;
use crate::error::{DrivingError, RoutingError};
use crate::routing::RemoteCredentials;

pub enum RemoteTransport {
    Ssh(RemoteSshTransport)
}

pub struct RemoteSshTransport {
    host: String,
    port: u16,
    credentials: RemoteCredentials,
    session: Option<AsyncSession<TcpStream>>,
}

impl RemoteSshTransport {
    pub async fn new(address: (String, u16), credentials: RemoteCredentials) -> Result<Self, DrivingError> {
        let mut transport = Self {
            host: address.0,
            port: address.1,
            credentials,
            session: None
        };

        transport.connect().await.map_err(|e| {
            let error = DrivingError::Routing(e);
            log::error!("DrivingError: {:?}", error);
            error
        })?;

        Ok(transport)
    }

    async fn connect(&mut self) -> Result<(), RoutingError> {
        let host = self.host.parse().map_err(|e| {
            let error = RoutingError::Address(e);
            log::error!("RoutingError: {:?}", error);
            error
        })?;

        let socket_address = SocketAddr::new(host, self.port);
        let session = AsyncSession::<async_ssh2_lite::TokioTcpStream>::connect(socket_address, None)
            .await
            .map_err(|e| {
                let error = RoutingError::Ssh(e);
                log::error!("RoutingError: {:?}", error);
                error
            })?;

        match &self.credentials {
            RemoteCredentials::Password(credentials) => {
                session.userauth_password(&credentials.username, &credentials.password)
                    .await
                    .map_err(|e| {
                        let error = RoutingError::Ssh(e);
                        log::error!("RoutingError: {:?}", error);
                        error
                    })?;
            }

            RemoteCredentials::Key(credentials) => {
                let private_key_path = PathBuf::from(&credentials.private_key);
                session.userauth_pubkey_file(&credentials.username, None, &private_key_path, None)
                    .await
                    .map_err(|e| {
                        let error = RoutingError::Ssh(e);
                        log::error!("RoutingError: {:?}", error);
                        error
                    })?;
            }
        }

        self.session = Some(session);
        Ok(())
    }
}

impl TransportTrait for RemoteSshTransport {
    fn read(&self) -> Result<Box<dyn Read>, DrivingError> {
        todo!()
    }

    fn write(&self, _reader: &mut Box<dyn Read>) -> Result<(), DrivingError> {
        todo!()
    }

    fn content_type(&self) -> Result<String, DrivingError> {
        todo!()
    }
}

impl fmt::Display for RemoteSshTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoteTransport")
    }
}
