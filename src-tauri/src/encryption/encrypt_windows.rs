use std::ptr;
use rand::prelude::*;
use base64::{engine::general_purpose, Engine};

pub fn encrypt_string(input: &str) -> String {
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

pub fn decrypt_string(input: &str) -> String {
    use winapi::um::{wincrypt::DATA_BLOB, dpapi::{CRYPTPROTECT_LOCAL_MACHINE, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData}, winbase::LocalFree};
    extern crate winapi;

    let decoded_data = match general_purpose::STANDARD_NO_PAD.decode(input) {
        Ok(data) => data,
        Err(_) => return "".to_string(), // Failed to decode base64
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
        return "".to_string()
    }

    let decrypted_data = unsafe {
        String::from_utf8_lossy(std::slice::from_raw_parts(data_blob_out.pbData, data_blob_out.cbData as usize)).to_string()
    };
    
    unsafe {
        LocalFree(data_blob_out.pbData as *mut _);
    }

    decrypted_data
}

pub fn generate_random_salt(length: i32) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    
    let mut rng = rand::thread_rng();
    let mut charset: Vec<char> = CHARSET.iter().map(|&b| b as char).collect();
    
    charset.shuffle(&mut rng);
    
    let salt: String = charset.into_iter().take(length.try_into().unwrap()).collect();

    salt
}