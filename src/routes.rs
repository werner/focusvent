use handlers::products;
use handlers::prices;
use handlers::costs;
use handlers::suppliers;
use handlers::clients;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![products::index,
            products::show,
            products::create,
            products::update,
            products::delete,
            prices::index,
            prices::show,
            prices::create,
            prices::update,
            prices::delete,
            suppliers::index,
            suppliers::show,
            suppliers::create,
            suppliers::update,
            suppliers::delete,
            clients::index,
            clients::show,
            clients::create,
            clients::update,
            clients::delete,
            costs::index,
            costs::show,
            costs::create,
            costs::update,
            costs::delete]
}
