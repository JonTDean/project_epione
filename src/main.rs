#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

mod auth;
mod controllers;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        controllers::auth_controller::register
    ])
}