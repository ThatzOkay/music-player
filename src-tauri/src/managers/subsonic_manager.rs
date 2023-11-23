use std::collections::HashMap;

use crate::managers::clients::subsonic_client::SubsonicClient;

use super::clients::responses::{subsonic_ping_response::SubsonicPingResponse, subsonic_search3_response::SubsonicSearch3Response};


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

    pub async fn search3(&self, query: &str) -> String {
        let mut additional_query_parans = HashMap::new();
        additional_query_parans.insert("query", query);
        
        let response = self.subsonic_client.get::<SubsonicSearch3Response>("search3", Some(additional_query_parans)).await;

        "".to_string()
    }

}