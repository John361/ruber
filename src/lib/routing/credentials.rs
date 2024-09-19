use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct RemoteCredentialsPassword {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RemoteCredentialsKey {
    pub username: String,
    pub private_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RemoteCredentials {
    Password(RemoteCredentialsPassword),
    Key(RemoteCredentialsKey),
}
