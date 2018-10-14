extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use focusvent::models::client;
use focusvent::models::sale::Sale;
use focusvent::models::currency::Currency;

fn create_currency(client: &Client) -> Currency {
    let mut response = client
        .post("/currencies")
        .header(ContentType::JSON)
        .body(r#"{
            "value": "Pesos ARS",
            "symbol": "$",
            "decimal_point": ",",
            "thousands_separator": ".",
            "default_currency": true,
            "in_use": true
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_client(client: &Client) -> client::Client {
    let mut response = client
        .post("/clients")
        .header(ContentType::JSON)
        .body(r#"{
            "first_name": "Jhon",
            "last_name": "Doe"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

pub fn create_sale(client: &Client) -> Sale {
    let currency = create_currency(client);
    let db_client = create_client(client);
    let mut response = client
        .post("/sales")
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "sale": {{
                "currency_id": {},
                "client_id": {},
                "sale_date": "2018-12-01"
            }},
            "sale_products": []
        }}"#, currency.id, db_client.id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}
