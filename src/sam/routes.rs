use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use rocket::request::Form;
use std::path::PathBuf;
use rocket::State;
use super::*;

#[get("/")]
fn index(context: State<SAM>) -> Template {
    Template::render("home", context.inner())
}

#[get("/crud/add/student")]
fn add_student() -> Template {
    // Read flash cookie and display it
    let context = Student::to_form();
    Template::render("crud_add", &context)
}

#[post("/crud/add/student", data = "<new_sdnt>")]
fn add_student_post(new_sdnt: Form<Student>) -> Redirect {
    use diesel::ExecuteDsl;
    use diesel::insert;
    use app::students;

    let conn = establish_connection();
    let sdnt = new_sdnt.get();

    insert(sdnt).into(students::table)
                .execute(&conn)
                .expect("Error adding new student");
    Redirect::to("/crud/add/student")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}
