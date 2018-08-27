use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use models::product_price::ProductPrice;
use models::price::Price;
use schema::product_prices::dsl::*;
use schema::prices::dsl::*;

#[derive(Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub stock: f64
}

impl Product {
    pub fn list(limit: i64, offset: i64) -> Result<Vec<(Product, (ProductPrice, Price))>, diesel::result::Error> {
        use schema::products::dsl::*;

        let connection = establish_connection();
        products
            .inner_join(product_prices.inner_join(prices))
            .limit(limit)
            .offset(offset)
            .load::<(Product, (ProductPrice, Price))>(&connection)
    }
}