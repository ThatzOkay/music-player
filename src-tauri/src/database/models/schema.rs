// @generated automatically by Diesel CLI.

diesel::table! {
    providers (id) {
        id -> Integer,
        name -> Text,
        api -> Text,
        username -> Text,
        password -> Text,
        ip -> Text,
        port -> Integer,
        connection_type -> Integer,
    }
}
