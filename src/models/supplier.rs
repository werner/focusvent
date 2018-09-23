use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::suppliers;
use basic_model_actions;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, AsChangeset, FromForm)]
pub struct Supplier {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: String,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="suppliers"]
pub struct NewSupplier {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: String,
    pub email: Option<String>,
    pub phone: Option<String>
}

basic_model_actions!(suppliers, Supplier, NewSupplier);
from_data!(Supplier);
from_data!(NewSupplier);
