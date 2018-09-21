use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::prices;
use schema::prices::dsl::*;
use basic_model_actions;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, Clone, AsChangeset, FromForm)]
pub struct Price {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="prices"]
pub struct NewPrice {
    pub name: String
}
 
basic_model_actions!(prices, Price, NewPrice);
from_data!(Price);
from_data!(NewPrice);
