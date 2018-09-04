use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::prices;
use schema::prices::dsl::*;
use taxonomy;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash)]
pub struct Price {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash)]
#[table_name="prices"]
pub struct NewPrice {
    pub name: String
}
 
taxonomy!(prices, Price, NewPrice);
from_data!(Price);
from_data!(NewPrice);