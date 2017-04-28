#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)] //Grrr... Rocket, please fix this!

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

pub mod sam;
use sam::*;
//code
fn main() {
    rocket::ignite().manage(SAM{}).mount("/", routes![index, files, add_student]).launch();
}
