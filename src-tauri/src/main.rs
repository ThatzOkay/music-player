// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use models::greet::Greet;

use std::ptr;

use base64::{Engine as _, engine::general_purpose};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Greet {
    let message = format!("Hello, {}! You've been greeted from Rust!", name);
    let encrypted_message = encrypt_string(&message);
    let decrypted_message = decrypt_string(&encrypted_message);

    let decrypted_greet_msg = match decrypted_message {
        Some(msg) => msg,
        None => {
            // Handle the case where decryption fails
            eprintln!("Failed to decrypt message!");
            // You can provide a default message or take other appropriate actions
            String::from("Decryption failed")
        }
    };

    Greet {
        greet_msg: message,
        encrypted_greet_msg: encrypted_message,
        decrypted_greet_msg,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
fn encrypt_string(input: &str) -> String {
    use winapi::um::{wincrypt::DATA_BLOB, dpapi::{CryptProtectData, CRYPTPROTECT_LOCAL_MACHINE, CRYPTPROTECT_UI_FORBIDDEN}, winbase::LocalFree};
    extern crate winapi;

    let mut data_blob_in = DATA_BLOB {
        cbData: input.len() as u32,
        pbData: input.as_ptr() as *mut _,
    };

    let mut data_blob_out = DATA_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };

    unsafe {
        CryptProtectData(
            &mut data_blob_in,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            CRYPTPROTECT_LOCAL_MACHINE | CRYPTPROTECT_UI_FORBIDDEN,
            &mut data_blob_out,
        );
    }

    if data_blob_out.cbData == 0 {
        eprintln!("Failed to encrypt data");
        return String::new();
    }

    let encrypted_data = unsafe {
        // Base64 encode the encrypted data
        let base64_encoded = general_purpose::STANDARD_NO_PAD.encode(std::slice::from_raw_parts(data_blob_out.pbData, data_blob_out.cbData as usize));

        // Convert the encoded data to a String
        String::from_utf8_lossy(base64_encoded.as_bytes()).to_string()
    };

    unsafe {
        LocalFree(data_blob_out.pbData as *mut _);
    }

    encrypted_data
}

#[cfg(target_os = "windows")]
fn decrypt_string(input: &str) -> Option<String> {
    use winapi::um::{wincrypt::DATA_BLOB, dpapi::{CRYPTPROTECT_LOCAL_MACHINE, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData}, winbase::LocalFree};
    extern crate winapi;

    let decoded_data = match general_purpose::STANDARD_NO_PAD.decode(input) {
        Ok(data) => data,
        Err(_) => return None, // Failed to decode base64
    };

    let mut data_blob_in = DATA_BLOB {
        cbData: decoded_data.len() as u32,
        pbData: decoded_data.as_ptr() as *mut _,
    };

    let mut data_blob_out = DATA_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };

    unsafe {
        CryptUnprotectData(
            &mut data_blob_in,
            core::ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            CRYPTPROTECT_LOCAL_MACHINE | CRYPTPROTECT_UI_FORBIDDEN,
            &mut data_blob_out,
        );
    }

    if data_blob_out.cbData == 0 {
        return None;
    }

    let decrypted_data = unsafe {
        String::from_utf8_lossy(std::slice::from_raw_parts(data_blob_out.pbData, data_blob_out.cbData as usize)).to_string()
    };
    
    unsafe {
        LocalFree(data_blob_out.pbData as *mut _);
    }

    Some(decrypted_data)
}