// This is a file for defining types and application constants and information.
// Also contains code for serializing data to and from different formats (Sqlite, CSV, etc.)
use util::*;

infer_schema!("dotenv:DATABASE_URL");

#[derive(Serialize)]
pub struct SAM {
    //Add actual state to this obviously
}

#[derive(Serialize)]
pub struct Form {
    post: String,
    fields: Vec<Input>,
    sub_text: String
}

impl Form {
    pub fn new(p: &str, f: Vec<Input>, s: &str) -> Form {
        Form {
            post: p.to_string(),
            fields: f,
            sub_text: s.to_string()
        }
    }
}

// Potentially replace this struct with a tuple and use .into instead of String::from()
#[derive(Serialize)]
pub struct Input {
    display_text: String,
    input_type: String,
    parse_field: String,
    options: Vec<String>,
}

impl Input {
    fn new(d: &str, i: &str, p: &str, o: Vec<String>) -> Input {
        Input { display_text: String::from(d)
              , input_type: String::from(i)
              , parse_field: String::from(p)
              , options: o
        }
    }
}

pub trait ToForm {
    fn to_form() -> Form;
}

#[derive(Queryable, Insertable)]
#[table_name="peaks"]
pub struct Peak {
    name: String,
}

#[derive(Queryable, Insertable, FromForm)]
#[table_name="students"]
pub struct Student {
    name: String,
    number: i32,
    year: i32,
    peak: String,
    secret: Option<String>,
}

impl ToForm for Student {
    fn to_form() -> Form {
        use diesel::prelude::*;
        use peaks::dsl::*;

        let connection = establish_connection();
        let peak_names = peaks.load::<Peak>(&connection)
                              .expect("Error loading peaks")
                              .into_iter()
                              .map(|x| x.name)
                              .collect();

        let mut form = Vec::new();
        form.push(Input::new("Name: ", "text", "name", Vec::new()));
        form.push(Input::new("Student Number: ", "number", "number", Vec::new()));
        form.push(Input::new("Graduating Class: ", "number", "year", Vec::new()));
        form.push(Input::new("Peak: ", "select", "peak", peak_names));
        Form::new("/crud/add/student", form, "Submit")
    }
}
