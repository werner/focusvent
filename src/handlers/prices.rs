use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::price::Price;
use models::price::NewPrice;
use models::price::BasicModelActions;

#[get("/prices?<params>")]
pub fn index(params: GetTransactionParams) -> Result<Json<Vec<Price>>, status::Custom<String>> {
    match Price::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0)) {
        Ok(prices) => Ok(Json(prices)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[post("/prices", format="application/json", data="<price>")]
pub fn create(price: NewPrice) -> Result<Json<Price>, status::Custom<String>> {
    match Price::create(price) {
        Ok(price) => Ok(Json(price)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[get("/prices/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<Price>, status::Custom<String>> {
    match Price::show(id) {
        Ok(price) => Ok(Json(price)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/prices/<id>", format="application/json", data="<price>")]
pub fn update(id: i32, price: Price) -> Result<Json<Price>, status::Custom<String>> {
    match Price::update(id, price) {
        Ok(price) => Ok(Json(price)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/prices/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Price::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}