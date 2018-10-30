use std::io::Read;
use std::default::Default;
use diesel;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::QueryDsl;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use models::db_connection::*;
use models::sale::Sale;

#[derive(Serialize, Deserialize, Debug, Clone, DbEnum)]
pub enum SaleStatus {
    Draft,
    Saved,
    Active,
    Cancelled,
    Payed,
    Overdue,
    Error
}

from_data!(SaleStatus);

impl Default for SaleStatus {
    fn default() -> SaleStatus { SaleStatus::Draft }
}

impl<'v> FromFormValue<'v> for SaleStatus {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<SaleStatus, &'v RawStr> {
        match form_value.as_str() {
            "draft" => Ok(SaleStatus::Draft),
            "saved" => Ok(SaleStatus::Saved),
            "active" => Ok(SaleStatus::Active),
            "cancelled" => Ok(SaleStatus::Cancelled),
            "payed" => Ok(SaleStatus::Payed),
            "overdue" => Ok(SaleStatus::Overdue),
            "error" => Ok(SaleStatus::Error),
            _ => Err(form_value)
        }
    }
}

impl SaleStatus {
    pub fn save_status(id: i32, sale_status: SaleStatus) -> Result<bool, diesel::result::Error> {
        use schema::sales::dsl;
        let connection = establish_connection();

        diesel::update(dsl::sales.find(id))
            .set(dsl::status.eq(sale_status))
            .get_result::<Sale>(&connection)?;
    
        Ok(true)
    }
}
