// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 30]
        login -> Varchar,
    }
}
