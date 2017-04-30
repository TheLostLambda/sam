#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)] //Grrr... Rocket, please fix this!

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate quick_csv as csv;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

pub mod sam;
use sam::*;
fn main() {
    //load_student_csv("/home/brady/Downloads/Miner_Cup_Student_Data_April2017.csv");
    rocket::ignite().manage(SAM{}).mount("/", routes![index, files, add_student, add_student_post]).launch();
}
