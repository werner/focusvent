use handlers::products;
use handlers::prices;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![products::index,
            prices::index,
            prices::create,
            prices::update,
            prices::delete]
}
