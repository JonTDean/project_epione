use diesel::{Queryable, prelude::Insertable};
use rocket::serde::Serialize;
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub pkey: Vec<u8>,
}