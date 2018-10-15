use chrono::NaiveDate;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(DieselNewType)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NaiveDateForm(NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        match form_value.parse() {
            Ok(naive_date) => Ok(NaiveDateForm(naive_date)),
            Err(_) =>  Err(form_value)
        }
    }
}
