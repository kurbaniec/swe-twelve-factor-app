// @generated automatically by Diesel CLI.

diesel::table! {
    datasets (id) {
        id -> Int4,
        in_use -> Bool,
        data -> Bytea,
        created_on -> Timestamp,
    }
}
