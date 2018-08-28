use handlers::products;
use handlers::prices;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![products::index,
            products::create,
            products::update,
            products::delete,
            prices::index,
            prices::create,
            prices::update,
            prices::delete]
}
