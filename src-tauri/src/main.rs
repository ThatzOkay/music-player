// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use encryption::platform_encryption::{decrypt_string, encrypt_string};
use models::greet::Greet;

mod encryption;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}