use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::price::Price;
use crate::models::price::SearchPrice;
use crate::models::price::NewPrice;
use crate::models::price::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("prices", Price, NewPrice, SearchPrice);
