// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod encryption;
mod enums;
mod helpers;
mod managers;
mod models;

use chrono::Utc;
use database::{
    database_manager::{self, DatabaseManager},
    models::models::{NewSong, Provider, Song},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use encryption::platform_encryption::encrypt_string;
use helpers::subsonic_credentials::SubsonicCredentials;
use managers::subsonic_manager::SubsonicManager;
use std::{
    fs,
    sync::{Arc, Mutex},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use enums::connection_type::ConnectionType;
use tauri::{Manager, Runtime};

fn init() -> bool {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let create_proj_dir_result = fs::create_dir_all(proj_dirs.config_dir());
        if create_proj_dir_result.is_err() {
            return false;
        }
        let conn_string = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let database_manager = DatabaseManager::new(conn_string);

        let mut connection = database_manager.establish_connection();
        let result = connection.run_pending_migrations(MIGRATIONS);
        if let Err(e) = result {
            println!("Error running migrations: {}", e);
        }

        return true;
    }

    return false;
}

#[tauri::command]
fn is_first_run() -> bool {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_stirng = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_stirng);
        let provider_count = database_manager.get_provider_count();

        if provider_count == 0 {
            return true;
        }

        return false;
    }

    return false;
}

#[tauri::command]
async fn is_first_scan() -> bool {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_stirng = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_stirng);
        let song_count = database_manager.get_song_count();

        if song_count == 0 {
            return true;
        }

        return false;
    }

    return false;
}

#[tauri::command]
async fn scan_providers<R: Runtime>(app: tauri::AppHandle<R>) -> Result<bool, String> {
    let binding = app.state::<Arc<Mutex<DatabaseManager>>>();

    // Clone the Arc inside the Mutex
    let database_manager_arc = binding.clone();

    // Lock the Mutex and get the providers
    let providers = {
        let mut guard = database_manager_arc
            .lock()
            .map_err(|_| "Failed to acquire lock on DatabaseManager")?;

        guard.get_providers()
    };

    let mut songs_to_add = Vec::<Song>::new();

    match providers {
        Ok(providers) => {
            for provider in providers {
                if let Some(conenction_type) =
                    ConnectionType::as_connection_type(provider.connection_type)
                {
                  let songs =  match conenction_type {
                        ConnectionType::Local => todo!(),
                        ConnectionType::Subsonic => scan_subsonic(provider).await
                    };
                    songs_to_add.extend(songs);
                }
            }
        }
        Err(err) => return Err(err),
    }

    let song_in_db_count = {
        let mut guard = database_manager_arc
            .lock()
            .map_err(|_| "Failed to acquire lock on DatabaseManager")?;

        guard.get_song_count()
    };

    if song_in_db_count == 0 {
        for song in songs_to_add {
            let added_song = {
                let mut guard = database_manager_arc
                    .lock()
                    .map_err(|_| "Failed to acquire lock on DatabaseManager")?;

                guard.add_song(song)
            };
        }
    }

    Ok(true)
}

async fn scan_subsonic(provider: Provider) -> Vec<Song> {
    let mut return_songs = Vec::<Song>::new();
    let provider_id = provider.id;
    let creds = SubsonicCredentials::from_provider(provider);

    let subsonic_manager = SubsonicManager::new(creds.host, creds.username, creds.password);

    let result = subsonic_manager.get_all_songs().await;

    if let Some(found_result) = result {
        let songs = found_result.song;

        if let Some(songs) = songs {
            songs.into_iter().for_each(|song| {
                let now = Utc::now().timestamp() as i32;

                let new_song: Song = Song {
                    id: 0,
                    created_at: now,
                    provider_id: provider_id.clone(),
                    updated_at: now,
                    provider_song_id: song.id,
                };

                return_songs.push(new_song);
            });
        }
    }

    return return_songs;
}

#[tauri::command]
async fn check_credentials(
    provider: ConnectionType,
    host: String,
    username: String,
    password: String,
) -> Result<bool, ()> {
    match provider {
        ConnectionType::Subsonic => {
            let subsonic_manager =
                SubsonicManager::new(host.as_str(), username.as_str(), password.as_str());

            let success = subsonic_manager.ping().await;

            Ok(success)
        }
        ConnectionType::Local => Ok(true),
    }
}

#[tauri::command]
fn add_provider<R: Runtime, 'a>(
    app: tauri::AppHandle<R>,
    provider: ConnectionType,
    host: &'a str,
    username: &'a str,
    password: &'a str,
) -> Result<Provider, String> {
    let binding = app.state::<Arc<Mutex<DatabaseManager>>>();

    let mut database_manager = binding
        .try_lock()
        .map_err(|_| "Failed to acquire lock on DatabaseManager")?;

    let _ = match database_manager.add_provider(
        &provider,
        host,
        encrypt_string(username).as_str(),
        encrypt_string(password).as_str(),
    ) {
        Ok(provider) => return Ok(provider), // Return Some(provider) on success
        Err(err) => return Err(err),         // Return None if an error occurs
    };
}

#[tauri::command]
fn get_providers<R: Runtime>(app: tauri::AppHandle<R>) -> Result<Vec<Provider>, String> {
    let binding = app.state::<Arc<Mutex<DatabaseManager>>>();

    let mut database_manager = binding
        .try_lock()
        .map_err(|_| "Failed to acquire lock on DatabaseManager")?;

    let providers_result = database_manager.get_providers();

    match providers_result {
        Ok(providers) => return Ok(providers),
        Err(err) => return Err(err),
    };
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
                let conn_string = proj_dirs
                    .config_dir()
                    .join("database.db")
                    .display()
                    .to_string();

                let db_manager = DatabaseManager::new(conn_string);

                let db_arc = Arc::new(Mutex::new(db_manager));
                app.manage(db_arc);
            }
            tauri::async_runtime::spawn(async move {
                init();

                std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            is_first_run,
            is_first_scan,
            add_provider,
            get_providers,
            scan_providers,
            check_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
