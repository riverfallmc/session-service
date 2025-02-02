// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        username -> Text,
        uuid -> Text,
        accesstoken -> Text,
        serverid -> Nullable<Text>,
    }
}
