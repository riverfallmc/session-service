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

diesel::table! {
    skincape_cache (name) {
        name -> Text,
        user_count -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        skin -> Nullable<Text>,
        cape -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    skincape_cache,
    users,
);
