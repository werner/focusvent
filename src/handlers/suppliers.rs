use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::supplier::Supplier;
use models::supplier::NewSupplier;
use models::supplier::Taxonomy;

#[get("/suppliers?<params>")]
pub fn index(params: GetTransactionParams) -> Result<Json<Vec<Supplier>>, status::Custom<String>> {
    match Supplier::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0)) {
        Ok(suppliers) => Ok(Json(suppliers)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[post("/suppliers", format="application/json", data="<supplier>")]
pub fn create(supplier: NewSupplier) -> Result<Json<Supplier>, status::Custom<String>> {
    match Supplier::create(supplier) {
        Ok(supplier) => Ok(Json(supplier)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[get("/suppliers/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<Supplier>, status::Custom<String>> {
    match Supplier::show(id) {
        Ok(supplier) => Ok(Json(supplier)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/suppliers/<id>", format="application/json", data="<supplier>")]
pub fn update(id: i32, supplier: Supplier) -> Result<Json<Supplier>, status::Custom<String>> {
    match Supplier::update(id, supplier) {
        Ok(supplier) => Ok(Json(supplier)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/suppliers/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Supplier::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}
