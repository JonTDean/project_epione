use diesel::RunQueryDsl;
use rocket::serde::json::{Json, Value};
use serde_json::json;
use crate::{controllers::auth_controller::RegisterData, models::User};
use crate::services::connection::establish_pg;
use uuid::Uuid;

use super::zk_snarks::snark_ops::{read_params_from_file, get_prepared_vk_from_params};
use super::zk_snarks::verifiers::verify_zk_proof;

/// Registers a new user.
/// Takes in the registration data and returns a JSON response indicating success or failure.
pub fn register_user(register_data: Json<RegisterData>) -> Json<Value> {
    // Validate the email address
    let valid_email = match validate_email(&register_data.email) {
        Ok(email) => email,
        Err(_) => return Json(json!({
            "status": "error",
            "message": "Invalid email address"
        }))
    };
    // Extract zk-SNARK proof and public inputs from the register_data
    let params_result = read_params_from_file();
    let params = match params_result {
        Ok(p) => p,
        Err(e) => {
            // Handle the error, e.g., log it and exit
            eprintln!("Failed to read parameters from file: {}", e);
            return Json(json!({"status": "error", "message": "Failed to read zk-SNARK parameters from file"}));        
        }
    };
    let pvk = get_prepared_vk_from_params(&params);
    let proof = &register_data.proof;  // Assuming the proof is stored directly in register_data
    let curr_public_inputs = &register_data.public_inputs;  // Similarly, extract public inputs
    
    // Verify the zk-SNARK proof
    match verify_zk_proof(&pvk, &proof, curr_public_inputs) {
        Ok(true) => {}, // The proof is valid, continue processing
        Ok(false) => return Json(json!({
            "status": "error",
            "message": "Invalid zk-SNARK proof"
        })),
        Err(_) => return Json(json!({
            "status": "error",
            "message": "Error verifying zk-SNARK proof"
        })),
    }

    // Create a new user instance
    let new_user = User {
        id: Uuid::new_v4(),
        email: valid_email,
        proof: register_data.proof.clone(),
    };

    // Register the user to the database and return the result
    register_to_database(new_user)
}

/// Validates the given email address.
/// Returns the email if valid, otherwise returns an error.
fn validate_email(email: &String) -> Result<String, ()> {
    if email.contains("@") {
        Ok(email.clone()) // Return the email if it contains an '@' symbol
    } else {
        Err(()) // Return an error if the email is invalid
    }
}

/// Registers the given user to the database.
/// Returns a JSON response indicating success or failure.
fn register_to_database(new_user: User) -> Json<Value> {
    // Import the database schema for the users table
    use crate::schema::users;

    // Establish a connection to the database
    let conn = &mut establish_pg();

    // Attempt to insert the new user into the database
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
    {
        // If insertion is successful, return a success response
        Ok(_) => Json(json!({
            "status": "success",
            "message": "User registered successfully"
        })),
        // If there's an error, return an error response with the error message
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    }
}