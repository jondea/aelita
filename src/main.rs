#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod cases;
use cases::CasesTemplate;

pub mod schema;
pub mod models;

use uuid::Uuid;

use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[get("/<name>")]
fn hello(name : String) -> HelloTemplate {
    HelloTemplate { name }
}

#[get("/")]
fn hello_no_one() -> HelloTemplate {
    hello(String::from("Mr Mysterious..."))
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/cases")]
fn show_cases() -> CasesTemplate {

    use schema::cases::dsl::*;

    let connection = establish_connection();
    let results = cases
        .limit(5)
        .load::<Case>(&connection)
        .expect("Error loading cases");

    return CasesTemplate{cases: results};
}

#[get("/cases/<id_str>")]
fn get_case(id_str: String) -> Result<Case, String> {

    use schema::cases::dsl::*;

    let get_id = match Uuid::parse_str(&id_str){
        Ok(i) => i, 
        Err(_e) => return Err(String::from("Invalid uuid")), 
    };

    let connection = establish_connection();
    let result = match cases.find(get_id).first::<Case>(&connection){
        Ok(r) => r, 
        Err(_e) => return Err(String::from("Couldn't find case")), 
    };

    return Ok(result);
}

fn main() {

    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/", routes![hello_no_one])
        .mount("/", routes![show_cases])
        .mount("/", routes![get_case])
        .launch();
}
