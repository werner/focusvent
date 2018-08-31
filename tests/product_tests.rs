extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate serde;
extern crate serde_json;

use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use focusvent::models::product::Product;
use focusvent::schema::products::dsl::*;
use focusvent::schema::prices::dsl::*;
use focusvent::schema::product_prices::dsl::*;

fn create_product(client: &Client) -> Product {
    let mut response = client
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
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_product_with_price(client: &Client) -> Product {
    let mut response = client
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
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

pub fn index(client: &Client, connection: &PgConnection) {
    diesel::delete(product_prices).execute(connection).unwrap();
    diesel::delete(products).execute(connection).unwrap();
    diesel::delete(prices).execute(connection).unwrap();

    let product = create_product(client);
    let product2 = create_product_with_price(client);
    let mut response = client.get("/products?offset=0&limit=10").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let string = format!(r#"[{{"id":{},"name":"Shoe","description":"for the feet","stock":0.0}},{{"id":{},"name":"Hat","description":"for the head","stock":0.0}}]"#,
                        product.id, product2.id);
    assert_eq!(Some(string), response.body_string());
}

pub fn show(client: &Client, connection: &PgConnection) {
    diesel::delete(product_prices).execute(connection).unwrap();
    diesel::delete(products).execute(connection).unwrap();
    diesel::delete(prices).execute(connection).unwrap();

    let product = create_product_with_price(client);
    let mut response = client.get(format!("/products/{}", product.id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(Some(format!(r#"{{"product":{{"id":{},"name":"Hat","description":"for the head","stock":0.0}},"prices":{{"default":1234,"max":5093}}}}"#, product.id)),
               response.body_string());
}