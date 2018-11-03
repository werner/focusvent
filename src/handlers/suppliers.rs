use crate::handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use crate::models::supplier::Supplier;
use crate::models::supplier::SearchSupplier;
use crate::models::supplier::NewSupplier;
use crate::models::supplier::BasicModelActions;
use crate::basic_handler_actions;

basic_handler_actions!("suppliers", Supplier, NewSupplier, SearchSupplier);
