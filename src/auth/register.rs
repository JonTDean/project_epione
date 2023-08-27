use diesel::RunQueryDsl;
use rocket::serde::json::{Json, Value};
use serde_json::json;
use crate::{controllers::auth_controller::RegisterData, models::User};
use crate::services::connection::establish_pg;

/// Registers a new user.
/// Takes in the registration data and returns a JSON response indicating success or failure.
pub fn register_user(register_data: Json<RegisterData>) -> Json<Value> {
    // Validate the email address
    let valid_email = match validate_email(&register_data.email) {
        // If email is valid, return the email
        Ok(email) => email,
        // If email is invalid, return an error response
        Err(_) => return Json(json!({
            "status": "error",
            "message": "Invalid email address"
        }))
    };

    // Create a new user instance
    let new_user = User {
        id: 0, // Temporary ID, database should auto-generate this
        email: valid_email,
        pkey: register_data.pkey.clone(), // Clone the public key from the registration data
    };

    // Save the user to the database and return the result
    save_to_database(new_user)
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

/// Saves the given user to the database.
/// Returns a JSON response indicating success or failure.
fn save_to_database(new_user: User) -> Json<Value> {
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