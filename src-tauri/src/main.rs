// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod encryption;
mod database;
mod enums;
mod managers;

use std::fs;
use managers::subsonic_manager::SubsonicManager;
use database::{database_manager::{DatabaseManager, self}, models::models::Provider};
use directories::ProjectDirs;
use encryption::platform_encryption::{decrypt_string, encrypt_string};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use enums::connection_type::ConnectionType;
use models::greet::Greet;
use tauri::Manager;


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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Greet {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
    let encrypted_message = encrypt_string(&message);
    let decrypted_message = decrypt_string(&encrypted_message);

    Greet {
        greet_msg: message,
        encrypted_greet_msg: encrypted_message,
        decrypted_greet_msg: decrypted_message,
    }
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
async fn check_credentials(provider: ConnectionType, host: String, username: String, password: String) -> Result<bool, ()> {
  match provider {ConnectionType::Subsonic=> {

    let subsonic_manager = SubsonicManager::new(host.as_str(), username.as_str(), password.as_str());

    let success = subsonic_manager.ping().await;

    Ok(success)
  },
    ConnectionType::Local => Ok(true)
}
}

#[tauri::command]
fn add_provider<'a>(provider: ConnectionType, host: &'a str, username: &'a str, password: &'a str) -> Result<Provider, String> {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_stirng = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_stirng);

        let _ = match database_manager.add_provider(&provider, host, encrypt_string(username).as_str(), encrypt_string(password).as_str()) {
            Ok(provider) => Ok(provider), // Return Some(provider) on success
            Err(_) => Err("".to_string()), // Return None if an error occurs
        };
    }
    
    return Err("".to_string());
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                init();
                std::thread::sleep(std::time::Duration::from_secs(1));
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, is_first_run, add_provider, check_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}