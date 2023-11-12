use std::ptr;

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

pub fn decrypt_string(input: &str) -> Option<String> {
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