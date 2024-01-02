use diesel::prelude::*;

use super::schema::{providers, songs};
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

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::models::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Song {
    pub id: i32,
    pub provider_id: i32,
    pub provider_song_id: String,
    pub created_at: i32,
    pub updated_at: i32
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

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong<'a> {
    pub provider_id: &'a i32,
    pub provider_song_id: &'a str,
    pub created_at: &'a i32,
    pub updated_at: &'a i32,
}