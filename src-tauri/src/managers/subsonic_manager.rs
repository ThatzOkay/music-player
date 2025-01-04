use std::collections::HashMap;

use crate::clients::response::subsonic_album_response::{Album, SubsonicAlbumResponse};
use crate::clients::subsonic_client::SubsonicClient;

use crate::clients::response::subsonic_ping_response::SubsonicPingResponse;
use crate::database::models::Provider;
use crate::encryption::platform_encryption::decrypt_string;


pub struct SubsonicManager<'a> {
    subsonic_client: SubsonicClient<'a>,
}

impl<'a> SubsonicManager<'a> {
    pub fn new(
        host: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> SubsonicManager<'a> {
        let subsonic_client = SubsonicClient::new(host, username, password);
        SubsonicManager { subsonic_client }
    }

    pub fn new_form_provider(provider: Provider) -> SubsonicManager<'static> {
        let host = Box::leak(Box::new(format!("{}:{}", provider.schema, provider.ip)));
        let username = Box::leak(Box::new(decrypt_string(&provider.username)));
        let password = Box::leak(Box::new(decrypt_string(&provider.password)));
    
        let subsonic_client = SubsonicClient::new(host, username, password);
        SubsonicManager { subsonic_client }
    }

    pub async fn ping(&self) -> bool {
        let response = self.subsonic_client.get::<SubsonicPingResponse>("ping", None).await;
        
        match response {
            Ok(response) => {
                if let Some(error) = response.subsonic_response.error {
                    eprintln!("code: {}, message: {}", error.code, error.message);
                    return false;
                }
                true
            },
            Err(err) => {
                eprint!("{}", err);
                false
            },
        }
    }

    pub async fn get_albums(&self) -> Result<Vec<Album>, String> {
        let mut extra_params = HashMap::new();
        extra_params.insert("type".to_string(), "random".to_string());

        let response = self.subsonic_client.get::<SubsonicAlbumResponse>("getAlbumList2", Some(extra_params)).await;

        match response {
            Ok(response) => {
                if let Some(error) = response.subsonic_response.error {
                    eprintln!("code: {}, message: {}", error.code, error.message);
                    return Err(error.message);
                }

                Ok(response.subsonic_response.album_list2.album)
            },
            Err(err) => {
                eprint!("{}", err);
                return Err(err);
            }
        }
    }

}