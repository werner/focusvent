use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::price::Price;
use models::price::SearchPrice;
use models::price::NewPrice;
use models::price::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("prices", Price, NewPrice, SearchPrice);
