extern crate rocket;
extern crate regex;

use regex::Regex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

pub fn index(client: Client) {
    let mut response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(r#"{
            "name": "Shoe",
            "description": "for the feet"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    response = client.get("/products?offset=0&limit=10").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let re = Regex::new(r#","name":"Shoe","description":"for the feet","stock":0.0},null"#).unwrap();
    assert!(re.is_match(&response.body_string().unwrap()));
}