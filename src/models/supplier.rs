use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use schema::suppliers;
use schema::suppliers::dsl::*;
use models::db_connection;

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

impl Supplier {

    pub fn list(limit: i64, offset: i64) -> Result<Vec<Supplier>, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        suppliers
            .limit(limit)
            .offset(offset)
            .load::<Supplier>(&connection)
    }

    pub fn create(new_supplier: NewSupplier) -> Result<Supplier, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::insert_into(suppliers::table)
            .values(&new_supplier)
            .get_result(&connection)
    }

    pub fn show(request_id: i32) -> Result<Supplier, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        suppliers
            .find(request_id)
            .get_result::<Supplier>(&connection)
    }

    pub fn update(param_id: i32, supplier: Supplier) -> Result<Supplier, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::update(suppliers.find(param_id))
            .set((first_name.eq(supplier.first_name),
                  last_name.eq(supplier.last_name),
                  company_name.eq(supplier.company_name),
                  phone.eq(supplier.phone),
                  email.eq(supplier.email)))
            .get_result::<Supplier>(&connection)
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::delete(suppliers.find(param_id))
            .execute(&connection)
    }
}

from_data!(Supplier);
from_data!(NewSupplier);