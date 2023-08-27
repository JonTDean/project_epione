// Import the rocket and rocket_contrib crates
use rocket::*;
use rocket::serde::{Serialize, Deserialize, json::Json};
use serde_json::Value;

use crate::auth::register::register_user;

// Define a struct for the registration data
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterData {
    pub email: String,
    pub proof: Vec<u8>,
}

// Define a function that handles the registration logic
// It takes a Json object as a parameter and returns a Json object as a response
#[post("/register", format = "json", data = "<register_data>")]
pub fn register(register_data: Json<RegisterData>) -> Json<Value> {
    register_user(register_data)
}
