#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod cases;

use askama::Template;
use serde_json::{Map, Value};

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

#[get("/cases")]
fn show_cases() -> cases::Case {
    let case = cases::example_case();
    match case {
        Ok(v) => v,
        Err(e) => {println!("error in show_cases {:?}", e); return cases::Case { _id: String::from(""), events:Vec::from([])}},
    }
}

fn main() {


    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/", routes![hello_no_one])
        .mount("/", routes![show_cases])
        .launch();
}
