use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::currency::Currency;
use models::currency::SearchCurrency;
use models::currency::NewCurrency;
use models::currency::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("currencies", Currency, NewCurrency, SearchCurrency);
