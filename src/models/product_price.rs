use std::io::Read;
use std::collections::HashMap;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::pg::PgConnection;
use schema::product_prices;
use schema::product_prices::dsl::*;
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

#[derive(PartialEq)]
enum Action {
    Create,
    Update
}

impl ProductPrice {
    pub fn batch_create(hash_prices: HashMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        ProductPrice::batch_action(Action::Create, hash_prices, raw_product_id)
    }

    pub fn batch_update(hash_prices: HashMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        ProductPrice::batch_action(Action::Update, hash_prices, raw_product_id)
    }

    fn batch_action(action: Action, hash_prices: HashMap<String, i32>, raw_product_id: i32) -> Result<bool, diesel::result::Error> {
        let connection = establish_connection();

        for (new_name_price, amount) in &hash_prices {
            let mut db_price: Option<Price> = None;
            match prices.filter(name.eq(&new_name_price)).first(&connection) {
                Ok(_price) => db_price = Some(_price),
                Err(_) => {
                    if let Ok(_price) = Price::create(NewPrice { name: (&new_name_price).to_string() }) {
                        db_price = Some(_price);
                    }
                }
            }

            if let Some(real_db_price) = db_price {
                if action == Action::Update {
                    let result_edit_price = 
                        product_prices
                            .filter(price_id.eq(real_db_price.id).and(product_id.eq(raw_product_id)))
                            .first::<ProductPrice>(&connection);

                    if let Ok(edit_price) = result_edit_price {
                        diesel::update(product_prices.find(edit_price.id))
                            .set(price.eq(*amount))
                            .get_result::<ProductPrice>(&connection)?;
                    } else {
                        ProductPrice::create_product_price(&connection, raw_product_id, real_db_price.id, *amount)?;
                    }
                }

                ProductPrice::create_product_price(&connection, raw_product_id, real_db_price.id, *amount)?;
            }
        }

        Ok(true)
    }

    fn create_product_price(connection: &PgConnection, param_product_id: i32, param_price_id: i32, param_price: i32) -> Result<ProductPrice, diesel::result::Error> {
        let new_price = NewProductPrice { product_id: param_product_id, price_id: param_price_id, price: param_price };

        diesel::insert_into(product_prices::table)
            .values(&new_price)
            .get_result::<ProductPrice>(connection)
    }
}

from_data!(ProductPrice);