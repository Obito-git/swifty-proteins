// @generated automatically by Diesel CLI.

diesel::table! {
    proteins (code) {
        code -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    proteins,
    users,
);
