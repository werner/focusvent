use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::product::FullProduct;
use crate::models::product::Product;
use crate::models::product::SearchProduct;
use crate::models::product::FullNewProduct;

#[get("/products?<params>")]
pub fn index(params: GetTransactionParams<SearchProduct>) -> Result<Json<Vec<Product>>, status::Custom<String>> {
    Product::list(params.limit.unwrap_or(10),
                  params.offset.unwrap_or(0),
                  params.search) 
        .map(|products| Json(products))
        .map_err(|error| status::Custom(Status::NotFound, error.to_string()))
}

#[get("/products/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<FullProduct>, status::Custom<String>> {
    Product::show(id)
        .map(|product| Json(product))
        .map_err(|error| status::Custom(Status::NotFound, error.to_string()))
}

#[post("/products", format="application/json", data="<request>")]
pub fn create(request: FullNewProduct) -> Result<Json<Product>, status::Custom<String>> {
    Product::create(request)
        .map(|product| Json(product))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[put("/products/<id>", format="application/json", data="<request>")]
pub fn update(id: i32, request: FullNewProduct) -> Result<Json<Product>, status::Custom<String>> {
    Product::update(id, request)
        .map(|product| Json(product))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[delete("/products/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    Product::delete(id)
        .map(|success| Json(success))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
