use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Privder {
    pub id: i32,
    pub name: String,
    pub api: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: i32,
    pub connection_type: i32
}