use std::io::Read;
use chrono::NaiveDate;
use diesel;
use diesel::prelude::*;
use models::naive_date_form::NaiveDateForm;
use schema::sales;

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Clone,
         Queryable, Debug, FromForm)]
#[table_name="sales"]
pub struct Sale {
    pub id: i32,
    pub client_id: i32,
    pub sale_date: NaiveDateForm,
    pub sub_total: i32,
    pub total: i32,
    pub observation: String
}

from_data!(Sale);