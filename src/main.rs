#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::Template;
use std::path::PathBuf;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> Template {
    let context: HashMap<usize,usize> = HashMap::new();
    Template::render("index", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}
