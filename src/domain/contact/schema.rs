// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        subject -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}
