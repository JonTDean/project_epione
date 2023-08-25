#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

mod auth;
mod controllers;



#[get("/debug")]
pub fn debug() -> &'static str {
    "Rocket is up and running!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            debug
        ])
        .mount("/auth", routes![
            controllers::auth_controller::register
        ])
}