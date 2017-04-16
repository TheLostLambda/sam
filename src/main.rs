#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)] //Grrr... Rocket, please fix this!
#![feature(custom_attribute)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate sam_meta;

pub mod sam;
use sam::*;

fn main() {
    rocket::ignite().manage(SAM{}).mount("/", routes![index, files]).launch();
}
