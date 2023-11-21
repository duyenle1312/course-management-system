// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        published -> Bool,
    }
}
