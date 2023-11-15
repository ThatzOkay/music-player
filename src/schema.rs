// @generated automatically by Diesel CLI.

diesel::table! {
    providers (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        api -> Nullable<Text>,
        ip -> Nullable<Text>,
        port -> Nullable<Integer>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        connection_type -> Nullable<Integer>,
    }
}
