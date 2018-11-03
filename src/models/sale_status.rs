use diesel;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::QueryDsl;
use crate::models::db_connection::*;
use crate::models::sale::Sale;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Clone, DbEnum)]
pub enum SaleStatus {
    Draft,
    Saved,
    Overdue,
    Cancelled,
    Payed,
}

impl Default for SaleStatus {
    fn default() -> SaleStatus {
        SaleStatus::Draft
    }
}

impl<'v> FromFormValue<'v> for SaleStatus {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<SaleStatus, &'v RawStr> {
        match form_value.as_str() {
            "draft" => Ok(SaleStatus::Draft),
            "saved" => Ok(SaleStatus::Saved),
            "overdue" => Ok(SaleStatus::Overdue),
            "cancelled" => Ok(SaleStatus::Cancelled),
            "payed" => Ok(SaleStatus::Payed),
            _ => Err(form_value),
        }
    }
}

impl SaleStatus {
    pub fn to_saved(id: i32) -> Result<bool, String> {
        Self::save_status(id, SaleStatus::Draft, SaleStatus::Saved)
    }

    pub fn to_cancelled(id: i32) -> Result<bool, String> {
        Self::save_status(id, SaleStatus::Saved, SaleStatus::Cancelled)
    }

    fn save_status(
        id: i32,
        previous_status: SaleStatus,
        next_status: SaleStatus,
    ) -> Result<bool, String> {
        use crate::schema::sales::dsl;
        let connection = establish_connection();

        match diesel::update(dsl::sales.find(id).filter(dsl::status.eq(previous_status)))
            .set(dsl::status.eq(next_status))
            .get_result::<Sale>(&connection)
        {
            Ok(_) => Ok(true),
            Err(_) => Err("Not valid State".to_string()),
        }
    }
}
