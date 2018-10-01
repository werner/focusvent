use diesel::expression::{ AppearsOnTable, Expression, NonAggregate };
use diesel::query_builder::{ AstPass, QueryFragment };
use diesel::sql_types::Date;
use diesel::pg::Pg;
use diesel::result::QueryResult;
use chrono::NaiveDate;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;
use diesel::deserialize::Queryable;

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

impl NonAggregate for NaiveDateForm { }

impl QueryFragment<Pg> for NaiveDateForm {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" DATE");
        Ok(())
    }
}

impl Queryable<Date, Pg> for NaiveDateForm {
    type Row = NaiveDate;

    fn build(row: Self::Row) -> Self {
        NaiveDateForm(row)
    }
}