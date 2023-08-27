use diesel::{Queryable, prelude::Insertable, Identifiable};
use rocket::serde::Serialize;
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Serialize, Insertable, Identifiable, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub proof: Vec<u8>,
}