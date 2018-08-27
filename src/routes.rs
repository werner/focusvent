use handlers::products;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![products::index]
}
