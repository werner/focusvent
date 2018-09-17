use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::costs;
use schema::costs::dsl::*;
use basic_model_actions;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, AsChangeset)]
pub struct Cost {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="costs"]
pub struct NewCost {
    pub name: String
}
 
basic_model_actions!(costs, Cost, NewCost);
from_data!(Cost);
from_data!(NewCost);
