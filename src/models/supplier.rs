use schema::suppliers;
use schema::suppliers::dsl::*;

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Supplier {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="suppliers"]
pub struct NewSupplier {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}