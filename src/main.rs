#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod handlers;
#[macro_use]
mod data_guards;
mod models;
mod routes;

fn main() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
