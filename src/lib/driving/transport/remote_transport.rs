use std::fmt;
use std::io::Read;
use std::net::SocketAddr;
use std::path::PathBuf;

use async_ssh2_lite::AsyncSession;
use tokio::net::TcpStream;
use crate::driving::TransportTrait;
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
    pub async fn new(address: (String, u16), credentials: RemoteCredentials) -> anyhow::Result<Self> {
        let mut transport = Self {
            host: address.0,
            port: address.1,
            credentials,
            session: None
        };

        transport.connect().await?;
        Ok(transport)
    }

    async fn connect(&mut self) -> anyhow::Result<()> {
        let socket_address = SocketAddr::new(self.host.parse()?, self.port);
        let session = AsyncSession::<async_ssh2_lite::TokioTcpStream>::connect(socket_address, None).await?;

        match &self.credentials {
            RemoteCredentials::Password(credentials) => {
                session.userauth_password(&credentials.username, &credentials.password).await?;
            }

            RemoteCredentials::Key(credentials) => {
                let private_key_path = PathBuf::from(&credentials.private_key);
                session.userauth_pubkey_file(&credentials.username, None, &private_key_path, None).await?;
            }
        }

        self.session = Some(session);
        Ok(())
    }
}

impl TransportTrait for RemoteSshTransport {
    fn read(&self) -> anyhow::Result<Box<dyn Read>> {
        todo!()
    }

    fn write(&self, _reader: &mut Box<dyn Read>) -> anyhow::Result<()> {
        todo!()
    }

    fn content_type(&self) -> anyhow::Result<String> {
        todo!()
    }
}

impl fmt::Display for RemoteSshTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoteTransport")
    }
}
