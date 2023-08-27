use diesel::{Queryable, prelude::Insertable};
use rocket::serde::Serialize;
use crate::schema::users;

#[derive(Queryable, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pkey: Vec<u8>,
}