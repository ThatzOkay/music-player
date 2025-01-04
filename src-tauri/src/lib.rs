mod clients;
mod database;
mod encryption;
mod enums;
mod managers;

use clients::response::subsonic_album_response::Album;
use database::{
    database_manager::{self, DatabaseManager},
    models::Provider,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use encryption::platform_encryption::encrypt_string;
use enums::connection_type::ConnectionType;
use managers::subsonic_manager::SubsonicManager;
use tauri::Manager;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn init() -> bool {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let create_proj_dir_result = std::fs::create_dir_all(proj_dirs.config_dir());
        if create_proj_dir_result.is_err() {
            eprintln!(
                "Error creating project directory: {}",
                create_proj_dir_result.err().unwrap()
            );
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
        if result.is_err() {
            eprintln!("Error running migrations: {}", result.err().unwrap());
            return false;
        }

        return true;
    }

    false
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn is_first_run() -> bool {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_string = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();

        let mut database_manager = DatabaseManager::new(conn_string);
        let provider_count = database_manager.get_provider_count();

        if provider_count == 0 {
            return true;
        }

        return false;
    }

    false
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
fn add_provider<'a>(
    provider: ConnectionType,
    host: &'a str,
    username: &'a str,
    password: &'a str,
) -> Result<Provider, String> {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_stirng = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_stirng);

        let _ = match database_manager.add_provider(
            &provider,
            host,
            encrypt_string(username).as_str(),
            encrypt_string(password).as_str(),
        ) {
            Ok(provider) => return Ok(provider), // Return added provider on success
            Err(err) => return Err(err),         // Return None if an error occurs
        };
    }

    Err("".to_string())
}

#[tauri::command]
async fn get_providers() -> Result<Vec<Provider>, String> {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_string = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_string);

        let providers = database_manager.get_providers();

        return Ok(providers);
    }

    Err("".to_string())
}

#[tauri::command]
async fn get_albums_for_provider(provider_id: i32) -> Result<Vec<Album>, String> {
    if let Some(proj_dirs) = ProjectDirs::from("nl", "thatzokay", "music-player") {
        let conn_string = proj_dirs
            .config_dir()
            .join("database.db")
            .display()
            .to_string();
        let mut database_manager = database_manager::DatabaseManager::new(conn_string);

        let provider = database_manager.get_provider_by_id(provider_id);

        if provider.is_none() {
            return Err("Provider not found".to_string());
        }

        let provider = provider.unwrap();
        let provider_clone = provider.clone();

        match ConnectionType::from_u32(provider.connection_type) {
            ConnectionType::Subsonic => {
                let subsonic_manager = SubsonicManager::new_form_provider(
                    provider_clone
                );

                let albums = subsonic_manager.get_albums().await;

                if albums.is_err() {
                    return Err("Error getting albums".to_string());
                }

                let albums = albums.unwrap();
                
                return Ok(albums);
            }
            ConnectionType::Local => {
                return Ok(Vec::new());
            }
        }
    }

    Err("".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                init();
                std::thread::sleep(std::time::Duration::from_secs(1));
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            is_first_run,
            check_credentials,
            add_provider,
            get_providers,
            get_albums_for_provider
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
