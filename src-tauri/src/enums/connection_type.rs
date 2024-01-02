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

    pub fn as_int(&self) -> i32 {
        match self {
            ConnectionType::Local => 0,
            ConnectionType::Subsonic => 1,
        }
    }

    pub fn as_connection_type(int: i32) -> Option<ConnectionType> {
        match int {
            0 => Some(ConnectionType::Local),
            1 => Some(ConnectionType::Subsonic),
            _ => None
        }
    }
}