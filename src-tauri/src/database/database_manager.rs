use diesel::prelude::*;

pub struct DatabaseManager {
    conn_string: String,
    connection: SqliteConnection,
}

impl DatabaseManager {
    pub fn new(conn_string: String) -> DatabaseManager{
        let connection = Self::establish_connection_with_conn_string(conn_string.clone());
        DatabaseManager {
            conn_string,
            connection,
        }
    }

    fn establish_connection_with_conn_string(conn_string: String) -> SqliteConnection {
        SqliteConnection::establish(&conn_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &conn_string))
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.conn_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.conn_string))
    }

    pub fn get_provider_count(&mut self) -> i32 {
        0
    }
}