// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Int8,
        size -> Int4,
        hash -> Text,
        owner_ref -> Int4,
        create_at -> Timestamp,
    }
}

diesel::table! {
    files (id) {
        id -> Int8,
        owner -> Int8,
        name -> Text,
        size -> Int4,
        parent -> Text,
        file_type -> Int4,
        create_at -> Timestamp,
        meta -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        phone -> Text,
        email -> Text,
        role -> Int4,
        status -> Int4,
        info -> Jsonb,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    files,
    users,
);
