use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize, Deserialize)]
pub enum ConnectionType {
    Local,
    Subsonic,
}

impl ConnectionType {
    pub fn get_name(&self) -> &str {
        match self {
            ConnectionType::Local => "Local",
            ConnectionType::Subsonic => "Subsonic",
        }
    }

    pub fn get_type(&self) -> i32 {
        match self {
            ConnectionType::Local => 0,
            ConnectionType::Subsonic => 1,
        }
    }
}