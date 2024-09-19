pub struct Route {
    pub source: Watch,
    pub destination: Watch,
}

pub enum Watch {
    Local(LocalWatch),
    Remote(RemoteWatch),
}

pub struct LocalWatch {
    pub name: String,
    pub folder: String,
}

pub struct RemoteWatch {
    pub name: String,
    pub address: String,
    pub folder: String,
    pub credentials: RemoteCredentials,
}

pub struct RemoteCredentialsPassword {
    pub username: String,
    pub password: String,
}

pub struct RemoteCredentialsKey {
    pub username: String,
    pub private_key: String,
}

pub enum RemoteCredentials {
    Password(RemoteCredentialsPassword),
    Key(RemoteCredentialsKey),
}
