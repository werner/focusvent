use std::io::Read;
use diesel::*;
use schema::product_prices;
use models::product::Product;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name="product_prices"]
#[belongs_to(Product)]
pub struct ProductPrice {
    pub id: i32,
    pub product_id: i32,
    pub price_id: i32,
    pub price: i32
}

impl ProductPrice {
    pub fn create() {
        unimplemented!();
    }
}

from_data!(ProductPrice);