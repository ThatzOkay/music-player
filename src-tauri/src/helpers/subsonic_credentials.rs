use crate::{database::models::models::Provider, encryption::platform_encryption::decrypt_string};


pub struct SubsonicCredentials<'a> {
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl SubsonicCredentials<'static> {
    pub fn from_provider(provider: Provider) -> SubsonicCredentials<'static> {
        let protocol = match provider.port {
            443 => "https://",
            _ => "http://",
        };

        let host = format!("{}{}", protocol, provider.ip);

        let username = decrypt_string(&provider.username);
        let password = decrypt_string(&provider.password);

        SubsonicCredentials {
            host: Box::leak(Box::new(host)),
            username: Box::leak(Box::new(username)),
            password: Box::leak(Box::new(password)),
        }
    }
}
