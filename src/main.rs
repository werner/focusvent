#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate serde;

#[macro_use]
extern crate serde_derive;

mod handlers;
mod models;
mod routes;

fn main() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
