use diesel::{prelude::*, result::Error};
use reqwest::Url;
use crate::enums::connection_type::ConnectionType;

use super::models::{models::{NewProvider, Provider}, schema::providers};

pub struct DatabaseManager {
    conn_string: String,
    connection: SqliteConnection,
}

impl DatabaseManager {
    pub fn new(conn_string: String) -> DatabaseManager{
        let connection = Self::establish_connection_with_conn_string(conn_string.clone());
        DatabaseManager {
            conn_string,
            connection,
        }
    }

    fn establish_connection_with_conn_string(conn_string: String) -> SqliteConnection {
        SqliteConnection::establish(&conn_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &conn_string))
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.conn_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.conn_string))
    }

    pub fn get_provider_count(&mut self) -> i32 {
        use crate::database::models::schema::providers::dsl::*;
        let count_result = providers.count().get_result::<i64>(&mut self.connection);
        match count_result {
            Ok(count) => count as i32,
            Err(err) => {
                eprintln!("Error fetching provider count: {}", err);
                0 // Return 0 when an error occurs
            }
        }
    }

    pub fn add_provider(&mut self, connection_type: &ConnectionType, host: &str, username: &str, password: &str) -> Result<Provider, String> {
        
        let url = match Url::parse(host) {
            Ok(uri) => uri,
            Err(err) => {
                eprintln!("Error parsing URI: {}", err);
                return Err(err.to_string());
            }
        };
        
        let (ip, port) = match url.scheme() {
            "http" => (url.host_str().unwrap_or_default().to_string(), 80),
            "https" => (url.host_str().unwrap_or_default().to_string(), 443),
            _ => {
                if let Some(host_with_port) = url.host_str() {
                    if let Some(pos) = host_with_port.find(":") {
                        let (ip_part, port_part) = host_with_port.split_at(pos);
                        let port = port_part[1..].parse::<i32>().unwrap_or(80);
                        (ip_part.to_string(), port)
                    } else {
                        (url.host_str().unwrap_or_default().to_string(), url.port().unwrap_or(80) as i32)
                    }
                } else {
                    ("".to_string(), 0)
                }
            }
        };

        let conn_name = connection_type.get_name();
        let conn_type = connection_type.get_type();

        let new_provider = NewProvider {connection_type: &(conn_type.clone() as i32), name: conn_name, api: conn_name, ip: ip.as_str(), port: &port, username, password };
        
        let result = diesel::insert_into(providers::table)
            .values(&new_provider)
            .execute(&mut self.connection);

            
        match result {
            Ok(_) => {
                let last_added_provider = self.get_last_added_provider();
                match last_added_provider {
                    Ok(provider) => return Ok(provider),
                    Err(err) => return Err(err.to_string())
                }
            }
            Err(err) => return Err(err.to_string()),
        };
    }

    pub fn get_last_added_provider(&mut self) -> Result<Provider, Error> {
        use super::models::schema::providers::dsl::*;
        let provider = providers.order(id.desc()).first::<Provider>(&mut self.connection);
        provider
    }
}