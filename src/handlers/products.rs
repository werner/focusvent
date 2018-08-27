use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::product::Product;
use models::price::Price;
use models::product_price::ProductPrice;

#[get("/products")]
pub fn index() -> Result<Json<Vec<(Product, (ProductPrice, Price))>>, status::Custom<String>> {
    match Product::list(10, 0) {
        Ok(products) => Ok(Json(products)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}