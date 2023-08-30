// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        proof -> Bytea,
        hashed_password -> Bytea,
        public_inputs -> Bytea,
    }
}
