extern crate focusvent;
extern crate diesel;
extern crate rocket;
extern crate regex;

#[cfg(test)]
mod test {
    use diesel;
    use diesel::RunQueryDsl;
    use rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use focusvent::models::db_connection::*;
    use focusvent::schema::products::dsl::*;

    fn rocket() -> rocket::Rocket {
        rocket::ignite().mount("/", ::focusvent::routes::routes())
    }

    #[test]
    fn main() {
        use regex::Regex;
        let client = Client::new(rocket()).expect("valid rocket instance");

        let connection = establish_connection();
        diesel::delete(products).execute(&connection).unwrap();

        let mut response = client
            .post("/products")
            .header(ContentType::JSON)
            .body(r#"{
                "name": "Shoe",
                "description": "for the feet"
            }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        response = client.get("/products").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let re = Regex::new(r#","name":"Shoe","description":"for the feet","stock":0.0},null"#).unwrap();
        assert!(re.is_match(&response.body_string().unwrap()));
    }
}