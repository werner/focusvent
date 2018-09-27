use diesel::expression::AppearsOnTable;
use diesel::expression::Expression;
use diesel::sql_types::Date;
use chrono::NaiveDate;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NaiveDateForm(NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        match form_value.parse() {
            Ok(naive_date) => Ok(NaiveDateForm(naive_date)),
            _ => Err(form_value),
        }
    }
}

impl Deref for NaiveDateForm{
    type Target = NaiveDate;

    fn deref(&self)-> &NaiveDate{
        &self.0
    }
}

impl<QS> AppearsOnTable<QS> for NaiveDateForm {}

impl Expression for NaiveDateForm {
    type SqlType = Date;
}