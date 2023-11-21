use std::collections::HashMap;

use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::encryption::platform_encryption::generate_random_salt;

pub struct  SubsonicClient<'a> {
    host: &'a str,
    username: &'a str,
    password: &'a str,
}

impl SubsonicClient<'_> {
    pub fn new<'a>(host: &'a str, username: &'a str, password: &'a str) -> SubsonicClient<'a>{
        SubsonicClient {host, username, password}
    }

    pub async fn execute<T>(&self, endpoint: &str, method: reqwest::Method, data: Option<&serde_json::Value>) -> Result<T, String> where T: DeserializeOwned {
        let client = reqwest::Client::new();

        let mut  url = Url::parse(&format!("{}/rest/{}", self.host, endpoint))
        .map_err(|err| {
            format!("Failed to parse host: {:?}", err).to_string()
        })?;

        let salt = generate_random_salt(6);
        
        let hashed_password = md5::compute(format!("{}{}", self.password, salt));
        let formatted_hashed = format!("{:x}", hashed_password);
        let formatted_hashed_str = &formatted_hashed[..];

        let mut default_query_params = HashMap::new();
        default_query_params.insert("u", self.username);
        default_query_params.insert("t", formatted_hashed_str);
        default_query_params.insert("s", salt.as_str());
        default_query_params.insert("c", "music-player");
        default_query_params.insert("f", "json");
        default_query_params.insert("v", "1.16.1");
        
        url.query_pairs_mut().extend_pairs(default_query_params);

        let request_builder = match method {
            reqwest::Method::GET => client.get(url),
            reqwest::Method::POST => client.post(url),
            _ => return Err(format!("Invalid HTTP method: {}", method).as_str().into())
        };

        let response = request_builder.send().await;
        match response {
            Ok(response) => {
                match response.status() {
                    reqwest::StatusCode::OK => {
                        match response.json::<T>().await {
                            Ok(parsed_object) => Ok(parsed_object),
                            Err(err) =>Err(format!("Failed to parse JSON response error: {}", err.to_string())),
                        }
                    }
                    _other => {
                        Err(format!("status code: {} message: {}", response.status(),response.text().await.unwrap()).to_string())
                    }
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, String> where T: DeserializeOwned {
        self.execute::<T>(endpoint, reqwest::Method::GET, None).await
    }
}