use diesel;
use diesel::prelude::*;
use models::db_connection::*;

#[derive(Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stock: Option<f64>
}

impl Product {
    pub fn list(limit: i64, offset: i64) -> Result<Vec<Product>, diesel::result::Error> {
        use schema::products::dsl::*;

        let connection = establish_connection();
        products
            .limit(limit)
            .offset(offset)
            .load::<Product>(&connection)
    }
}