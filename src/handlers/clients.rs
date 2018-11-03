use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::client::Client;
use crate::models::client::SearchClient;
use crate::models::client::NewClient;
use crate::models::client::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("clients", Client, NewClient, SearchClient);
