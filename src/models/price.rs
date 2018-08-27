use std::io::Read;
use diesel;
use diesel::prelude::*;
use models::db_connection::*;
use schema::prices;
use schema::prices::dsl::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name="prices"]
pub struct Price {
    pub id: i32,
    pub name: String
}

impl Price {
    pub fn list(limit: i64, offset: i64) -> Result<Vec<Price>, diesel::result::Error> {
        let connection = establish_connection();

        prices
            .limit(limit)
            .offset(offset)
            .load::<Price>(&connection)
    }

    pub fn create(price: Price) -> Result<Price, diesel::result::Error> {
        let connection = establish_connection();

        diesel::insert_into(prices::table)
            .values(&price)
            .get_result(&connection)
    }

    pub fn update(param_id: i32, price: Price) -> Result<Price, diesel::result::Error> {
        use schema::prices::dsl::name;
        let connection = establish_connection();

        diesel::update(prices.find(param_id))
            .set(name.eq(price.name))
            .get_result::<Price>(&connection)
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        let connection = establish_connection();

        diesel::delete(prices.find(param_id))
            .execute(&connection)
    }
}

from_data!(Price);