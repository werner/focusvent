use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::product::Product;

#[get("/products")]
pub fn index() -> Result<Json<Vec<Product>>, status::Custom<String>> {
    match Product::list(10, 0) {
        Ok(products) => Ok(Json(products)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}