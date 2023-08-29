use rocket::*;
use rocket::serde::{Serialize, Deserialize, json::Json};
use serde_json::Value;

use crate::auth::register::register_user;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterData {
    pub email: String,
    pub proof: Vec<u8>,
    pub hashed_password: Vec<u8>,  // The hash of the password
    pub public_inputs: Vec<u8>, // Public inputs for zk-SNARK proof verification
    // You may also need to include additional fields for the zk-SNARK proof depending on your setup
}

// Define a function that handles the registration logic
// It takes a Json object as a parameter and returns a Json object as a response
#[post("/register", format = "json", data = "<register_data>")]
pub fn register(register_data: Json<RegisterData>) -> Json<Value> {
    register_user(register_data)
}

