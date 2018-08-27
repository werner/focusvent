use std::io::Read;
use diesel::*;
use schema::product_prices;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="product_prices"]
pub struct ProductPrice {
    pub id: i32,
    pub product_id: i32,
    pub price_id: i32,
    pub price: i32
}

from_data!(ProductPrice);