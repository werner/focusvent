extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate regex;

mod product_tests;

#[cfg(test)]
mod test {
    use diesel;
    use diesel::RunQueryDsl;
    use rocket;
    use rocket::local::Client;
    use focusvent::models::db_connection::*;
    use focusvent::schema::products::dsl::*;
    use product_tests;

    fn rocket() -> rocket::Rocket {
        rocket::ignite().mount("/", ::focusvent::routes::routes())
    }

    #[test]
    fn main() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        let connection = establish_connection();
        diesel::delete(products).execute(&connection).unwrap();
        product_tests::index(client);
    }
}