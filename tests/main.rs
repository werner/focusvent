extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate regex;

mod product_tests;
mod price_tests;
mod sale_tests;

#[cfg(test)]
mod test {
    use diesel;
    use diesel::pg::PgConnection;
    use diesel::RunQueryDsl;
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
        clear(&connection);
        price_tests::update(&client);
        clear(&connection);
        product_tests::index(&client);
        clear(&connection);
        product_tests::index_search(&client);
        clear(&connection);
        product_tests::update(&client);
        clear(&connection);
        product_tests::update_price_and_cost(&client);
        clear(&connection);
        product_tests::show(&client);
        clear(&connection);
        sale_tests::failed_creating_sale_with_no_sale_products(&client);

        clear(&connection);
        let product = product_tests::create_product_with_price(&client);
        sale_tests::show(&product, &client);
    }

    fn clear(connection: &PgConnection) {
        use focusvent::schema::prices::dsl::*;
        use focusvent::schema::product_prices::dsl::*;
        use focusvent::schema::product_costs::dsl::*;
        use focusvent::schema::costs::dsl::*;
        use focusvent::schema::suppliers::dsl::*;
        use focusvent::schema::sale_products::dsl::*;
        use focusvent::schema::products::dsl::*;
        use focusvent::schema::sales::dsl::*;

        diesel::delete(product_costs).execute(connection).unwrap();
        diesel::delete(costs).execute(connection).unwrap();
        diesel::delete(suppliers).execute(connection).unwrap();
        diesel::delete(product_prices).execute(connection).unwrap();
        diesel::delete(sale_products).execute(connection).unwrap();
        diesel::delete(products).execute(connection).unwrap();
        diesel::delete(prices).execute(connection).unwrap();
        diesel::delete(sales).execute(connection).unwrap();
    }
}
