// This is a file for defining types and application constants and information.
// Also contains code for serializing data to and from different formats (Sqlite, CSV, etc.)

infer_schema!("dotenv:DATABASE_URL");

#[derive(Serialize)]
pub struct SAM {
    //Add actual state to this obviously
}

type Form = Vec<Input>;

pub struct Input {
    display_name: String,
    input_type: String,
    parse_field: String,
    options: Vec<String>,
}

trait ToForm {
    fn to_form() -> Form;
}

#[derive(Queryable, Insertable)]
#[table_name="peaks"]
pub struct Peak {
    name: String,
}

#[derive(Queryable)]
pub struct Student {
    name: String,
    number: i32,
    year: i32,
    peak: String,
    secret: Option<String>,
}

#[derive(Insertable, FromForm)]
#[table_name="students"]
pub struct NewStudent {
    name: String,
    number: i32,
    year: i32,
    peak: String,
}
