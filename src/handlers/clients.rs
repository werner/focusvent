use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::client::Client;
use models::client::SearchClient;
use models::client::NewClient;
use models::client::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("clients", Client, NewClient, SearchClient);
