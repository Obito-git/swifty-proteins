// @generated automatically by Diesel CLI.

diesel::table! {
    file_metadata (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    proteins (code) {
        code -> Text,
        file_metadata_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(proteins -> file_metadata (file_metadata_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_metadata,
    proteins,
    users,
);
