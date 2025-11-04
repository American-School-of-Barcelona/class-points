// @generated automatically by Diesel CLI.

diesel::table! {
    records (rowid) {
        rowid -> Integer,
        change -> Integer,
        reason -> Text,
        date -> Text,
        student -> Integer,
    }
}

diesel::table! {
    students (id) {
        id -> Integer,
        name -> Text,
        points -> Integer,
    }
}

diesel::joinable!(records -> students (student));

diesel::allow_tables_to_appear_in_same_query!(records, students,);
