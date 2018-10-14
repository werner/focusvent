extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate regex;

mod product_tests;
mod price_tests;
mod sale_tests;

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::Client;
    use focusvent::models::db_connection::*;
    use product_tests;
    use price_tests;
    use sale_tests;

    fn rocket() -> rocket::Rocket {
        rocket::ignite().mount("/", ::focusvent::routes::routes())
    }

    #[test]
    fn main() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        let connection = establish_connection();
        price_tests::index(&client, &connection);
        price_tests::update(&client);
        product_tests::index(&client, &connection);
        product_tests::index_search(&client, &connection);
        product_tests::update(&client, &connection);
        product_tests::update_price_and_cost(&client, &connection);
        product_tests::show(&client, &connection);
        sale_tests::create_sale(&client);
    }
}
