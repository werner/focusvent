use rocket::response::status;
use rocket_contrib::Json;
use models::product::Product;

#[get("/products")]
pub fn index() -> Result<Json<Vec<Product>>, status::Custom<String>> {
    Ok(Json(vec![Product]))
}