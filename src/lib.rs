#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod handlers;
pub mod routes;
pub mod schema;
#[macro_use]
pub mod data_guards;
pub mod models;