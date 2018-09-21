use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::tax::Tax;
use models::tax::NewTax;
use models::tax::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("taxes", Tax, NewTax);
