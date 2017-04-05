#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let context: HashMap<usize,usize> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
