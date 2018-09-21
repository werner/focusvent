use handlers::products;
use handlers::prices;
use handlers::costs;
use handlers::suppliers;
use handlers::clients;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    vec![clients::index_route(),
         clients::show_route()]
    // routes![products::index,
    //         products::show,
    //         products::create,
    //         products::update,
    //         products::delete,
    //         prices::index,
    //         prices::show,
    //         prices::create,
    //         prices::update,
    //         prices::delete,
    //         suppliers::index,
    //         suppliers::show,
    //         suppliers::create,
    //         suppliers::update,
    //         suppliers::delete,
    //         clients::show,
    //         clients::create,
    //         clients::update,
    //         clients::delete,
    //         costs::index,
    //         costs::show,
    //         costs::create,
    //         costs::update,
    //         costs::delete]
}
