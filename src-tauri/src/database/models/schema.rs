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

diesel::table! {
    songs (id) {
        id -> Integer,
        provider_id -> Integer,
        provider_song_id -> Text,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(songs -> providers (provider_id));

diesel::allow_tables_to_appear_in_same_query!(
    providers,
    songs,
);
