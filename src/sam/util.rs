//This file will contain miscellaneous, helper functions and reexport common modules

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use csv::Csv;
use std::env;
use super::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

//This is a really nasty, single-use chunk of code. I would advise looking the other way.
pub fn load_student_csv(file: &str) {
    use diesel::ExecuteDsl;
    use diesel::insert;
    use app::students;

    let conn = establish_connection();

    let csv = Csv::from_file(file).unwrap();
    for row in csv.into_iter() {
        let row = row.unwrap();
        if let Ok((number, last_name, first_name, grade, peak)) = row.decode::<(i32, String, String, i32, String)>() {
            let name = first_name + " " + &last_name;
            let year = 2017 + (12 - grade); //Ew, 2017 is a magic number
            let peak = (&peak[0..1]).to_string() + &peak[1..].to_lowercase();
            let sdnt = Student { name, number, year, peak, secret: None };
            insert(&sdnt).into(students::table).execute(&conn).expect("Couldn't insert student");
        }
    }
}
