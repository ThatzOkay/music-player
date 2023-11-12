use base64::{engine::general_purpose, Engine};
extern crate security_framework;
use security_framework::base::SecKeychainItem;
use security_framework::os::macos::keychain::{SecKeychain, SecKeychainSettings};
use security_framework::os::macos::item::{SecKey, SecItem};
use security_framework::os::macos::cvt;

fn generate_or_retrieve_key() -> SecKey {
    let keychain = SecKeychain::create(window::app_name().expect("Failed to get app name"), SecKeychainSettings::default()).unwrap();

    let query = SecItem::query().class(SecKey::class()).keychain(keychain);
    if let Ok(existing_keys) = cvt(query.result()) {
        if let Some(key) = existing_keys.into_iter().next() {
            return key;
        }
    }

    let new_key = SecKey::generate(
        &keychain,
        security_framework::os::macos::key::KeyPairType::RSA,
        2048,
    )
    .expect("Failed to generate key pair");

    new_key
}

pub fn encrypt_string(input: &str) -> String {
    let key = generate_or_retrieve_key();
    let ciphertext = key.encrypt(security_framework::base::Padding::PKCS1, plaintext.as_bytes()).unwrap();
    general_purpose::STANDARD_NO_PAD.encode(&ciphertext)
}

pub fn decrypt_string(input: &str) -> Option<String> {
    let decoded_ciphertext = general_purpose::STANDARD_NO_PAD.decode(ciphertext).expect("Failed to base64 decode");
    let plaintext = key.decrypt(security_framework::base::Padding::PKCS1, &decoded_ciphertext).unwrap();
    String::from_utf8_lossy(&plaintext).into_owned()
}