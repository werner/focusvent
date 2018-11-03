use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::cost::Cost;
use crate::models::cost::SearchCost;
use crate::models::cost::NewCost;
use crate::models::cost::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("costs", Cost, NewCost, SearchCost);
