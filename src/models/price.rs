use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::prices;
use schema::prices::dsl::*;
use models::product_price::ProductPrice;
use taxonomy;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, Clone)]
pub struct Price {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="prices"]
pub struct NewPrice {
    pub name: String
}
 
taxonomy!(prices, Price, NewPrice);
from_data!(Price);
from_data!(NewPrice);
