#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/", routes![hello_no_one])
        .launch();
}