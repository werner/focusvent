use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::cost::Cost;
use models::cost::NewCost;
use models::cost::BasicModelActions;

#[get("/costs?<params>")]
pub fn index(params: GetTransactionParams) -> Result<Json<Vec<Cost>>, status::Custom<String>> {
    match Cost::list(params.limit.unwrap_or(10), params.offset.unwrap_or(0)) {
        Ok(costs) => Ok(Json(costs)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[post("/costs", format="application/json", data="<cost>")]
pub fn create(cost: NewCost) -> Result<Json<Cost>, status::Custom<String>> {
    match Cost::create(cost) {
        Ok(cost) => Ok(Json(cost)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[get("/costs/<id>", format="application/json")]
pub fn show(id: i32) -> Result<Json<Cost>, status::Custom<String>> {
    match Cost::show(id) {
        Ok(cost) => Ok(Json(cost)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[put("/costs/<id>", format="application/json", data="<cost>")]
pub fn update(id: i32, cost: Cost) -> Result<Json<Cost>, status::Custom<String>> {
    match Cost::update(id, cost) {
        Ok(cost) => Ok(Json(cost)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/costs/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Cost::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}
