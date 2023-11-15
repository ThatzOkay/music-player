// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod encryption;
mod database;

use std::fs;
use database::database_manager::{DatabaseManager, self};
use directories::ProjectDirs;
use encryption::platform_encryption::{decrypt_string, encrypt_string};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use models::greet::Greet;

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
            .join("database.sqlite")
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

fn main() {
    init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, is_first_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}