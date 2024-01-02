use chrono::Utc;
use diesel::{prelude::*, result::Error, update};
use reqwest::Url;
use crate::enums::connection_type::ConnectionType;

use super::models::{models::{NewProvider, Provider, Song, NewSong}, schema::{providers, songs}};

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

    pub fn get_provider(&mut self, provider_id: i32) -> Result<Provider, String> {
        use crate::database::models::schema::providers::dsl::*;
        let found_provider = providers.find(provider_id).first::<Provider>(&mut self.connection);

        match found_provider {
            Ok(found_provider) => return Ok(found_provider),
            Err(err) => Err(err.to_string())
        }
    }

    pub fn get_providers(&mut self) -> Result<Vec<Provider>, String> {
        use crate::database::models::schema::providers::dsl::*;
        let return_providers = providers
        .load::<Provider>(&mut self.connection);
        match return_providers {
            Ok(return_providers) => return Ok(return_providers),
            Err(err) => Err(err.to_string()),
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
        let conn_type = connection_type.as_int();

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

    pub fn get_song_count(&mut self) -> i32 {
        use crate::database::models::schema::songs::dsl::*;
        let count_result = songs.count().get_result::<i64>(&mut self.connection);
        match count_result {
            Ok(count) => count as i32,
            Err(err) => {
                eprint!("Error fetching song count: {}", err);
                0 // Return 0 when error occurs
            }
        }
    }

    pub fn get_song(&mut self, song_id: i32) -> Result<Option<Song>, String> {
        use crate::database::models::schema::songs::dsl::*;

        let song_result = songs.filter(id.eq(song_id))
            .first::<Song>(&mut self.connection)
            .optional(); 

        match song_result {
            Ok(song) => return Ok(song),
            Err(err) => {
                Err(err.to_string())
            }
        }
    }

    pub fn get_songs(&mut self) -> Result<Vec<Song>, String> {
        use crate::database::models::schema::songs::dsl::*;
        let return_songs = songs
        .load::<Song>(&mut self.connection);
        match return_songs {
            Ok(return_songs) => return Ok(return_songs),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn add_song(&mut self, song: Song) -> Result<Song, String> {
        let new_song = NewSong {
            created_at: &song.created_at,
            provider_id: &song.provider_id,
            provider_song_id: &song.provider_song_id,
            updated_at: &song.updated_at
        };

        let result = diesel::insert_into(songs::table)
            .values(new_song)
            .execute(&mut self.connection);

        match result {
            Ok(_) => {
                let last_added_song = self.get_last_added_song();
                match last_added_song {
                    Ok(song) => return Ok(song),
                    Err(err) => return Err(err.to_string())
                }
            },
            Err(err) => return Err(err.to_string())
        };
    }

    pub fn get_last_added_song(&mut self) -> Result<Song, Error> {
        use super::models::schema::songs::dsl::*;

        let song = songs.order(id.desc()).first::<Song>(&mut self.connection);
        song
    }

    pub fn update_song(&mut self, song: Song) -> Result<Song, String> {
        use crate::database::models::schema::songs::dsl::*;

        let existing_song_result = self.get_song(song.id);

        match existing_song_result {
            Ok(song_option) => {
                if song_option.is_none() {
                    return Err("Song does not exist".to_string());
                }
                
                let existing_song = song_option.unwrap();
                let now = Utc::now().timestamp() as i32;

                let updated_song = Song {
                    id: existing_song.id,
                    provider_id: song.provider_id,
                    provider_song_id: song.provider_song_id,
                    created_at: existing_song.created_at,
                    updated_at: now,
                };

                let row_updated = update(songs.filter(id.eq(existing_song.id)))
                    .set(&updated_song)
                    .execute(&mut self.connection);

                match row_updated {
                    Ok(_) => {
                        let db_song = self.get_song(updated_song.id);
                        
                        if db_song.is_ok() {
                            return Ok(db_song.unwrap().unwrap())
                        }

                    },
                    Err(err) => {
                        return Err(err.to_string())
                    }
                }

                return Ok(updated_song);
            },
            Err(err) => return Err(err.to_string()),
        }

    }
}