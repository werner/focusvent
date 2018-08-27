#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod handlers;
mod models;

fn main() {
    println!("Hello, world!");
}
