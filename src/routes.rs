use handlers::products;
use handlers::prices;
use handlers::costs;
use handlers::suppliers;
use handlers::clients;
use handlers::currencies;
use handlers::taxes;
use handlers::sales;
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
            currencies::index_route(),
            currencies::show_route(),
            currencies::create_route(),
            currencies::update_route(),
            currencies::delete_route(),
            prices::index_route(),
            prices::show_route(),
            prices::create_route(),
            prices::update_route(),
            prices::delete_route(),
            suppliers::index_route(),
            suppliers::show_route(),
            suppliers::create_route(),
            suppliers::update_route(),
            suppliers::delete_route(),
            taxes::index_route(),
            taxes::show_route(),
            taxes::create_route(),
            taxes::update_route(),
            taxes::delete_route()];

    let mut automatic_routes =
        routes![
            products::index,
            products::show,
            products::create,
            products::update,
            products::delete,
            sales::index,
            sales::show,
            sales::create,
            sales::update,
            sales::delete,
            sales::set_status,
        ];
    
    manual_routes.append(&mut automatic_routes);

    manual_routes

}
