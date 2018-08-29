use std::io::Read;
use std::collections::HashMap;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use schema::product_prices;
use schema::prices::dsl::*;
use models::price::NewPrice;
use models::price::Price;
use models::db_connection::*;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Debug)]
pub struct ProductPrice {
    pub id: i32,
    pub product_id: i32,
    pub price_id: i32,
    pub price: i32
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="product_prices"]
pub struct NewProductPrice {
    pub product_id: i32,
    pub price_id: i32,
    pub price: i32
}

impl ProductPrice {
    pub fn batch_create(hash_prices: HashMap<NewPrice, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        let connection = establish_connection();

        for (new_price, amount) in &hash_prices {
            let mut db_price: Option<Price> = None;
            match prices.filter(name.eq(&new_price.name)).first(&connection) {
                Ok(_price) => db_price = Some(_price),
                Err(_) => {
                    if let Ok(_price) = Price::create(NewPrice { name: (&new_price.name).to_string() }) {
                        db_price = Some(_price);
                    }
                }
            }

            if let Some(real_db_price) = db_price {
                let new_price = NewProductPrice { product_id: raw_product_id, price_id: real_db_price.id, price: *amount };

                diesel::insert_into(product_prices::table)
                    .values(&new_price)
                    .get_result::<ProductPrice>(&connection)?;
            }
        }

        Ok(true)
    }
}

from_data!(ProductPrice);