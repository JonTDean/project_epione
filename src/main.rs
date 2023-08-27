#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

mod auth;
mod controllers;
mod schema;
mod models;
mod services;


#[get("/debug")]
pub fn debug() -> &'static str {
    "Rocket is up and running!"
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build()
        .mount("/", routes![
            debug
        ])
        .mount("/auth", routes![
            controllers::auth_controller::register
        ])
}