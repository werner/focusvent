use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::sale::Sale;
use crate::models::sale::FullSale;
use crate::models::sale::FullNewSale;
use crate::models::sale::SearchSale;
use crate::models::sale_status::SaleStatus;

#[get("/sales?<params>")]
pub fn index(params: GetTransactionParams<SearchSale>) -> Result<Json<Vec<Sale>>, status::Custom<String>> {
    Sale::list(params.limit.unwrap_or(10),
               params.offset.unwrap_or(0),
               params.search)
        .map(|sales| Json(sales))
        .map_err(|error| status::Custom(Status::NotFound, error.to_string()))
}

#[get("/sales/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<FullSale>, status::Custom<String>> {
    Sale::show(id)
        .map(|sale| Json(sale))
        .map_err(|error| status::Custom(Status::NotFound, error.to_string()))
}

#[post("/sales", format="application/json", data="<sale>")]
pub fn create(sale: FullNewSale) -> Result<Sale, status::Custom<String>> {
    Sale::create(sale)
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[put("/sales/<id>", format="application/json", data="<sale>")]
pub fn update(id: i32, sale: FullNewSale) -> Result<Json<Sale>, status::Custom<String>> {
    Sale::update(id, sale)
        .map(|sale| Json(sale))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[put("/sales/<id>/save", format="application/json")]
pub fn save(id: i32) -> Result<Json<bool>, status::Custom<String>> {
    SaleStatus::to_saved(id)
        .map(|success| Json(success))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[put("/sales/<id>/cancel", format="application/json")]
pub fn cancel(id: i32) -> Result<Json<bool>, status::Custom<String>> {
    SaleStatus::to_cancelled(id)
        .map(|success| Json(success))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}

#[delete("/sales/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    Sale::delete(id)
        .map(|success| Json(success))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
