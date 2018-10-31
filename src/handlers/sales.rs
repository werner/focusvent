use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::sale::Sale;
use models::sale::FullSale;
use models::sale::FullNewSale;
use models::sale::SearchSale;
use models::sale_status::SaleStatus;

#[get("/sales?<params>")]
pub fn index(params: GetTransactionParams<SearchSale>) -> Result<Json<Vec<Sale>>, status::Custom<String>> {
    match Sale::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0), params.search) {
        Ok(sales) => Ok(Json(sales)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[get("/sales/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<FullSale>, status::Custom<String>> {
    match Sale::show(id) {
        Ok(sale) => Ok(Json(sale)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[post("/sales", format="application/json", data="<sale>")]
pub fn create(sale: FullNewSale) -> Result<Json<Sale>, status::Custom<String>> {
    match Sale::create(sale.clone()) {
        Ok(sale) => Ok(Json(sale)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/sales/<id>", format="application/json", data="<sale>")]
pub fn update(id: i32, sale: FullNewSale) -> Result<Json<Sale>, status::Custom<String>> {
    match Sale::update(id, sale) {
        Ok(sale) => Ok(Json(sale)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/sales/<id>/save", format="application/json")]
pub fn save(id: i32) -> Result<Json<bool>, status::Custom<String>> {
    match SaleStatus::to_saved(id) {
        Ok(success) => Ok(Json(success)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/sales/<id>/cancel", format="application/json")]
pub fn cancel(id: i32) -> Result<Json<bool>, status::Custom<String>> {
    match SaleStatus::to_cancelled(id) {
        Ok(success) => Ok(Json(success)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/sales/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Sale::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}
