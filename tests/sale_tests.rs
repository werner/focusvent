extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use focusvent::models::money::Money;
use focusvent::models::client;
use focusvent::models::product::Product;
use focusvent::models::sale::Sale;
use focusvent::models::sale::FullSale;
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

pub fn failed_creating_sale_with_no_sale_products(client: &Client) {
    let currency = create_currency(client);
    let db_client = create_client(client);
    let response = client
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
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

pub fn create_sale(product: &Product, client: &Client) -> Sale {
    let currency = create_currency(client);
    let db_client = create_client(client);
    let mut response = client
        .post("/sales")
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "sale": {{
                "currency_id": {},
                "client_id": {},
                "sale_date": "2018-12-02"
            }},
            "sale_products": [{{
                "product_id": {},
                "tax": "12.0",
                "amount": 2.0,
                "price": "5.0"
            }}]
        }}"#, currency.id, db_client.id, product.id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

pub fn show(product: &Product, client: &Client) {
    let sale = create_sale(&product, client);
    let mut response = client.get(format!("/sales/{}", sale.id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let full_sale: FullSale = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    println!("{:?}", full_sale);
    assert_eq!("Jhon", &full_sale.sale.client().unwrap().first_name.unwrap());
    assert_eq!(Money(10), full_sale.sale.total);
}
