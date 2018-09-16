use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::suppliers;
use schema::suppliers::dsl::*;
use taxonomy;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, AsChangeset)]
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

taxonomy!(suppliers, Supplier, NewSupplier);
from_data!(Supplier);
from_data!(NewSupplier);
