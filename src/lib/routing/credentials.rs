use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RemoteCredentialsPassword {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoteCredentialsKey {
    pub username: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub enum RemoteSshCredentials {
    Password(RemoteCredentialsPassword),
    Key(RemoteCredentialsKey),
}
