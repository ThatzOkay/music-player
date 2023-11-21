use diesel::prelude::*;

use super::schema::providers;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::models::schema::providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub api: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: i32,
    pub connection_type: i32
}

#[derive(Insertable)]
#[diesel(table_name = providers)]
pub struct NewProvider<'a> {
    pub connection_type: &'a i32,
    pub name: &'a str,
    pub api: &'a str,
    pub ip: &'a str,
    pub port: &'a i32,
    pub username: &'a str,
    pub password: &'a str,
}