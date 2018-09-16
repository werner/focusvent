use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::costs;
use schema::costs::dsl::*;
use taxonomy;

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
 
taxonomy!(costs, Cost, NewCost);
from_data!(Cost);
from_data!(NewCost);
