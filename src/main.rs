#![feature(decl_macro)]
#[macro_use] extern crate rocket;

mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        controllers::auth_controller::generate,
        controllers::auth_controller::verify
    ])
}