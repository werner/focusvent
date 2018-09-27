use std;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;

pub struct Search<S>(S);

#[derive(FromForm)]
pub struct GetTransactionParams<S: std::str::FromStr> {
    pub search: Option<Search<S>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl<'v, S: std::str::FromStr> FromFormValue<'v> for Search<S> {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Search<S>, &'v RawStr>
    where S: std::str::FromStr
    {
        match form_value.parse::<S>() {
            Ok(search) => Ok(Search(search)),
            _ => Err(form_value),
        }
    }
}

impl<S> Deref for Search<S> {
    type Target = S;

    fn deref(&self) -> &S {
        &self.0
    }
}
