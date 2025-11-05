// @generated automatically by Diesel CLI.

diesel::table! {
    records (rowid) {
        rowid -> Integer,
        change -> Integer,
        reason -> Text,
        date -> Text,
        student -> Integer,
        points -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        points -> Integer,
        role -> Integer,
        password -> Binary,
        salt -> Binary,
    }
}

diesel::allow_tables_to_appear_in_same_query!(records, users,);
