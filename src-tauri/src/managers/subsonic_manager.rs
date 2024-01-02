use std::collections::HashMap;

use crate::managers::clients::subsonic_client::SubsonicClient;

use super::clients::responses::{subsonic_ping_response::SubsonicPingResponse, subsonic_search3_response::{SubsonicSearch3Response, SearchResult3, Song, Artist, Album}, self};


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

    pub async fn search3(&self, query: &str, artist_count: i32, artist_offset: i32, album_count: i32, album_offset: i32, song_count: i32, song_offset: i32) -> Option<SearchResult3> {
        let mut additional_query_params = HashMap::new();
        let artist_count_str = artist_count.to_string();
        let artist_count_str_ref: &str = artist_count_str.as_str();
        let artist_offset_str = artist_offset.to_string();
        let artist_offset_str_ref: &str = &artist_offset_str.as_str();
        let album_count_str = album_count.to_string();
        let album_count_str_ref: &str = &album_count_str.as_str();
        let album_offset_str = album_offset.to_string();
        let album_offset_str_ref: &str = &album_offset_str.as_str();
        let song_count_str = song_count.to_string();
        let song_count_str_ref: &str = &song_count_str.as_str();
        let song_offset_str = song_offset.to_string();
        let song_offset_str_ref: &str = &song_offset_str.as_str();

        additional_query_params.insert("query", query);
        additional_query_params.insert("artistCount", artist_count_str_ref);
        additional_query_params.insert("artistOffset", artist_offset_str_ref);
        additional_query_params.insert("albumCount", album_count_str_ref);
        additional_query_params.insert("albumOffset", album_offset_str_ref);
        additional_query_params.insert("songCount", song_count_str_ref);
        additional_query_params.insert("songOffset", song_offset_str_ref);
    
        let response = self.subsonic_client.get::<SubsonicSearch3Response>("search3", Some(additional_query_params)).await;
    
        match response {
            Ok(response) => {
                if let Some(error) = response.subsonic_response.error {
                    eprintln!("code: {}, message: {}", error.code, error.message);
                    return None;
                }
    
                // Introduce a variable to hold the search_result option
                let search_result_option = response.subsonic_response.search_result3;
    
                if let Some(search_result) = search_result_option {
                    return Some(search_result);
                }
    
                return None;
            },
            Err(err) => {
                eprint!("{}", err);
                None
            },
        }
    }
    

    pub async fn get_all_songs(&self) -> Option<SearchResult3> {
        let offset = 0;
        let mut songs = Vec::<Song>::new();

        loop {
            let response = self.search3("\"\"", 0, 0, 0, 0, 500, offset).await;
            
            if response.is_none() {
                break;
            }
            
            let songs_options = response.unwrap().song;

            if songs_options.is_none() {
                break;
            }

            let found_songs = songs_options.unwrap();

            let song_count = found_songs.len();
            let cloned_songs: Vec<Song> = found_songs;
            
            songs.extend(cloned_songs);

            if song_count < 500 {
                break;
            }
        }

        Some(SearchResult3 {
            album: None,
            artist: None,
            song: Some(songs)
        })
    }

}