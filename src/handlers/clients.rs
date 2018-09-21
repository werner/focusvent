use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::client::Client;
use models::client::NewClient;
use models::client::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("clients", Client);

#[post("/clients", format="application/json", data="<client>")]
pub fn create(client: NewClient) -> Result<Json<Client>, status::Custom<String>> {
    match Client::create(client) {
        Ok(client) => Ok(Json(client)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

// #[get("/clients/<id>", format="application/json")]
// pub fn show(id: i32) -> Result<Json<Client>, status::Custom<String>> {
//     match Client::show(id) {
//         Ok(client) => Ok(Json(client)),
//         Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
//     }
// }

#[put("/clients/<id>", format="application/json", data="<client>")]
pub fn update(id: i32, client: Client) -> Result<Json<Client>, status::Custom<String>> {
    match Client::update(id, client) {
        Ok(client) => Ok(Json(client)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}

#[delete("/clients/<id>", format="application/json")]
pub fn delete(id: i32) -> Result<Json<usize>, status::Custom<String>> {
    match Client::delete(id) {
        Ok(qid) => Ok(Json(qid)),
        Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
    }
}
