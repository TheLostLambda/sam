use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::PathBuf;
use rocket::State;
use super::*;

#[get("/")]
fn index(context: State<SAM>) -> Template {
    Template::render("home", context.inner())
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}
