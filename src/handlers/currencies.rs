use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::currency::Currency;
use crate::models::currency::SearchCurrency;
use crate::models::currency::NewCurrency;
use crate::models::currency::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("currencies", Currency, NewCurrency, SearchCurrency);
