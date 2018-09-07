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
use focusvent::models::product::FullProduct;
use focusvent::models::cost::Cost;
use focusvent::models::supplier::Supplier;
use focusvent::schema::products::dsl::*;

fn create_price(client: &Client) -> Cost {
    let mut response = client
        .post("/prices")
        .header(ContentType::JSON)
        .body(r#"{
            "name": "Default"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_price_2(client: &Client) -> Cost {
    let mut response = client
        .post("/prices")
        .header(ContentType::JSON)
        .body(r#"{
            "name": "Good"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_cost(client: &Client) -> Cost {
    let mut response = client
        .post("/costs")
        .header(ContentType::JSON)
        .body(r#"{
            "name": "Cheap"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_cost_2(client: &Client) -> Cost {
    let mut response = client
        .post("/costs")
        .header(ContentType::JSON)
        .body(r#"{
            "name": "Expensive"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_supplier(client: &Client) -> Supplier {
    let mut response = client
        .post("/suppliers")
        .header(ContentType::JSON)
        .body(r#"{
            "company_name": "My Company"
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_product(client: &Client) -> Product {
    let mut response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(r#"{
            "product": {
                "name": "Shoe",
                "description": "for the feet"
            },
            "prices": [],
            "costs": []
        }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

fn create_product_with_price(client: &Client) -> Product {
    let cost = create_cost(client);
    let cost2 = create_cost_2(client);
    let price = create_price(client);
    let price2 = create_price_2(client);
    let supplier = create_supplier(client);
    let mut response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "product": {{
                "name": "Hat",
                "description": "for the head"
            }},
            "prices": [
                {{
                    "price_id": {},
                    "price": 2000
                }},
                {{
                    "price_id": {},
                    "price": 1000
                }}
            ],
            "costs": [
                {{
                    "cost_id": {},
                    "supplier_id": {},
                    "cost": 1234
                }},
                {{
                    "cost_id": {},
                    "supplier_id": {},
                    "cost": 5678
                }}
            ]
        }}"#, price.id, price2.id, cost.id, supplier.id, cost2.id, supplier.id))
        .dispatch();
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

pub fn update(client: &Client) {
    let product = create_product(client);
    let response = client
        .put(format!("/products/{}", product.id))
        .header(ContentType::JSON)
        .body(format!(r#"{{
            "product": {{
                "id": {},
                "name": "Shoes",
                "description": "for the feet"
            }},
            "prices": [],
            "costs": []
        }}"#, product.id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut response = client.get(format!("/products/{}", product.id)).dispatch();
    assert_eq!(Some(format!(r#"{{"product":{{"id":{},"name":"Shoes","description":"for the feet","stock":0.0}},"prices":[],"costs":[]}}"#, product.id)),
               response.body_string());
}

pub fn index(client: &Client, connection: &PgConnection) {
    use focusvent::schema::prices::dsl::*;
    use focusvent::schema::product_prices::dsl::*;

    use focusvent::schema::product_costs::dsl::*;
    use focusvent::schema::costs::dsl::*;
    use focusvent::schema::suppliers::dsl::*;

    diesel::delete(product_costs).execute(connection).unwrap();
    diesel::delete(costs).execute(connection).unwrap();
    diesel::delete(suppliers).execute(connection).unwrap();
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
    use focusvent::schema::product_costs::dsl::*;
    use focusvent::schema::prices::dsl::*;
    use focusvent::schema::product_prices::dsl::*;

    diesel::delete(product_costs).execute(connection).unwrap();
    diesel::delete(product_prices).execute(connection).unwrap();
    diesel::delete(products).execute(connection).unwrap();
    diesel::delete(prices).execute(connection).unwrap();

    let product = create_product_with_price(client);
    let mut response = client.get(format!("/products/{}", product.id)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let full_product: FullProduct = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!("Hat", full_product.product.name);
    assert_eq!(Some("for the head".to_string()), full_product.product.description);
    assert_eq!(2, full_product.prices.len());
    assert_eq!(2, full_product.costs.len());
}