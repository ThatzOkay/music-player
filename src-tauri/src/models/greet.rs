use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize, Deserialize)]
pub struct Greet {
    #[serde(rename = "greetMsg")]
    pub greet_msg: String,
    #[serde(rename = "encryptedGreetMsg")]
    pub encrypted_greet_msg: String,
    #[serde(rename = "decryptedGreetMsg")]
    pub decrypted_greet_msg: String
}