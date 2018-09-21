use handlers::products;
use handlers::prices;
use handlers::costs;
use handlers::suppliers;
use handlers::clients;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    let mut manual_routes = 
        vec![clients::index_route(),
            clients::show_route(),
            clients::create_route(),
            clients::update_route(),
            clients::delete_route(),
            costs::index_route(),
            costs::show_route(),
            costs::create_route(),
            costs::update_route(),
            costs::delete_route(),
            prices::index_route(),
            prices::show_route(),
            prices::create_route(),
            prices::update_route(),
            prices::delete_route(),
            suppliers::index_route(),
            suppliers::show_route(),
            suppliers::create_route(),
            suppliers::update_route(),
            suppliers::delete_route()];

    let mut automatic_routes =
        routes![products::index,
                products::show,
                products::create,
                products::update,
                products::delete];
    
    manual_routes.append(&mut automatic_routes);

    manual_routes

}
