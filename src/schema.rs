// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        pkey -> Bytea,
    }
}