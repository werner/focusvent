use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::tax::Tax;
use crate::models::tax::SearchTax;
use crate::models::tax::NewTax;
use crate::models::tax::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("taxes", Tax, NewTax, SearchTax);
