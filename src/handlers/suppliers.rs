use handlers::base::GetTransactionParams;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
use models::supplier::Supplier;
use models::supplier::NewSupplier;
use models::supplier::BasicModelActions;
use basic_handler_actions;

basic_handler_actions!("suppliers", Supplier, NewSupplier);