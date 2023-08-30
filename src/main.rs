#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

mod auth;
mod controllers;
mod schema;
mod models;
mod services;

use auth::zk_snarks::{DebuggableParameters, snark_setup::ensure_zksnark_params, snark_ops::read_params_from_file};

#[get("/debug")]
pub fn debug() -> &'static str {
    "Rocket is up and running!"
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    // Ensure zk-SNARK parameters exist
    ensure_zksnark_params();
    let params = read_params_from_file().unwrap();
    // println!("{:#?}", DebuggableParameters(params));
    DebuggableParameters(params).save_to_files().expect("Failed to save to files");

    rocket::build()
        .mount("/", routes![
            debug
        ])
        .mount("/auth", routes![
            controllers::auth_controller::register
        ])
}