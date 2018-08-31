extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate regex;

use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use regex::Regex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use focusvent::schema::products::dsl::*;
use focusvent::schema::prices::dsl::*;
use focusvent::schema::product_prices::dsl::*;

fn create_product(client: &Client) {
    let response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(r#"{
            "product": {
                "name": "Shoe",
                "description": "for the feet"
            },
            "prices": {}
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

fn create_product_with_price(client: &Client) {
    let response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(r#"{
            "product": {
                "name": "Hat",
                "description": "for the head"
            },
            "prices": {
                "default": 1234,
                "max": 5093
            }
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

pub fn index(client: Client, connection: &PgConnection) {
    diesel::delete(product_prices).execute(connection).unwrap();
    diesel::delete(products).execute(connection).unwrap();
    diesel::delete(prices).execute(connection).unwrap();

    create_product(&client);
    create_product_with_price(&client);
    let mut response = client.get("/products?offset=0&limit=10").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let re = Regex::new(r#""name":"Shoe","description":"for the feet","stock":0.0},.*"name":"Hat","description":"for the head","stock":0.0}]"#).unwrap();
    assert!(re.is_match(&response.body_string().unwrap()));
}