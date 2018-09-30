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

use focusvent::models::price::Price;

fn create_price(client: &Client, name: String) -> Price {
    let mut response = client
        .post("/prices")
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "name": "{}"
        }}"#, name))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

pub fn update(client: &Client) {
    let _price = create_price(client, "Cheap".to_string());
    let response = client
        .put(format!("/prices/{}", _price.id))
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "id": {},
            "name": "Better"
        }}"#, _price.id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut response = client.get(format!("/prices/{}", _price.id)).dispatch();
    assert_eq!(Some(format!(r#"{{"id":{},"name":"Better"}}"#, _price.id)),
               response.body_string());
}

pub fn index(client: &Client, connection: &PgConnection) {
    use focusvent::schema::product_prices::dsl::*;
    use focusvent::schema::prices::dsl::*;

    diesel::delete(product_prices).execute(connection).unwrap();
    diesel::delete(prices).execute(connection).unwrap();

    let _price = create_price(client, "Cheap".to_string());
    create_price(client, "Expensive".to_string());
    let price3 = create_price(client, "Cheapest".to_string());
    create_price(client, "Less".to_string());
    let mut response = client.get("/prices?offset=0&limit=10&search={\"name\": \"Cheap%\"}").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let string = format!(r#"[{{"id":{},"name":"Cheap"}},{{"id":{},"name":"Cheapest"}}]"#,
                        _price.id, price3.id);
    assert_eq!(Some(string), response.body_string());
}
