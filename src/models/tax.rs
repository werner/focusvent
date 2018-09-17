use std::io::Read;
use diesel;
use diesel::prelude::*;
use schema::taxes;
use schema::taxes::dsl::*;
use basic_model_actions;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, Clone, AsChangeset)]
#[table_name="taxes"]
pub struct Tax {
    pub id: i32,
    pub name: String,
    pub percentage: i32
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="taxes"]
pub struct NewTax {
    pub name: String,
    pub percentage: i32
}

basic_model_actions!(taxes, Tax, NewTax);
from_data!(Tax);
from_data!(NewTax);
