use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::product::Product;
use models::product::FullNewProduct;
use models::price::Price;
use models::product_price::ProductPrice;

#[get("/products?<params>")]
pub fn index(params: GetTransactionParams) -> Result<Json<Vec<(Product, Option<(ProductPrice, Price)>)>>, status::Custom<String>> {
    match Product::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0)) {
        Ok(products) => Ok(Json(products)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[post("/products", format="application/json", data="<product>")]
pub fn create(product: FullNewProduct) -> Result<Json<Product>, status::Custom<String>> {
    match Product::create(product) {
        Ok(product) => Ok(Json(product)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/products/<id>", format="application/json", data="<product>")]
pub fn update(id: i32, product: Product) -> Result<Json<Product>, status::Custom<String>> {
    match Product::update(id, product) {
        Ok(product) => Ok(Json(product)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/products/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Product::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}