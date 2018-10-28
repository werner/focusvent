#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(print_internals)]
#![feature(fmt_internals)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate diesel_derive_newtype;
#[macro_use]
extern crate diesel_derive_enum;

mod schema;
mod handlers;
#[macro_use]
mod data_guards;
#[macro_use]
mod models;
mod routes;

fn main() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
