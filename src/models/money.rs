use std::{ fmt, str };
use std::num::ParseFloatError;
use serde::de::{ self, Deserialize, Deserializer, Visitor, Unexpected };
use serde::ser::{ Serialize, Serializer };
use diesel::expression::{ AppearsOnTable, Expression };
use diesel::query_builder::{ AstPass, QueryFragment };
use diesel::result::QueryResult;
use diesel::sql_types::Integer;
use diesel::Queryable;
use diesel::pg::Pg;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use models::currency::Currency;

#[derive(Clone, Debug)]
pub struct Money(i32);

impl Money {
    fn new(value: i32) -> Self {
        Money(value)
    }

    fn to_i32(value: &str, currency: &Currency) -> Result<i32, ParseFloatError> {
        let replaced_value = value.replace(&currency.decimal_point, ".");
        let float_value = replaced_value.parse::<f64>()?;
        Ok((float_value * 100.0).round() as i32)
    }

    fn to_f64_string(&self, currency: &Currency) -> String {
        let float_value = (self.0  as f64) / 100.0;
        let string_value = format!("{}", float_value);
        string_value.replace(".", &currency.decimal_point)
    }

    fn from_f64(value: f64) -> Self {
        Money((value * 100.0).round() as i32)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MoneyVisitor;

        impl<'de> Visitor<'de> for MoneyVisitor {
            type Value = Money;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Money type")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let currency = Currency::get_currency();

                let parsed_value = Money::to_i32(value, &currency)
                    .map_err(|_val| de::Error::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Money::new(parsed_value))
            }

        }

        deserializer.deserialize_str(MoneyVisitor)
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let currency = Currency::get_currency();

        serializer.serialize_str(&format!("{}", &self.to_f64_string(&currency)))
    }
}

impl Expression for Money {
    type SqlType = Integer;
}

impl<QS> AppearsOnTable<QS> for Money {}

impl<'v> FromFormValue<'v> for Money {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Money, &'v RawStr> {
        match ::serde_json::from_str(form_value) {
            Ok(money) => Ok(money),
            _ => Err(form_value),
        }
    }
}

impl Queryable<Integer, Pg> for Money {
    type Row = i32;

    fn build(row: Self::Row) -> Self {
        Money(row)
    }
}

impl QueryFragment<Pg> for Money {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" INTEGER");
        Ok(())
    }
}

/* Arithmetic Operations */
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;
use std::ops::Div;
use std::iter::Sum;

impl Mul for Money {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Money(self.0 * other.0)
    }
}

impl Mul<f64> for Money {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self::from_f64(self.0 as f64 * other)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Money(self.0 * other.0)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money(self.0 + other.0)
    }
}

impl<'a> Add<&'a Money> for Money {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        Money(self.0 + other.0)
    }
}

impl Div for Money {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Money(self.0 / other.0)
    }
}

impl Div<f64> for Money {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        let value = self.0 as f64 / 100.0;
        Self::from_f64(value / other)
    }
}

impl<'a> Sum<&'a Money> for Money {
    fn sum<I: Iterator<Item=&'a Money>>(iter: I) -> Money {
        iter.fold(Money(0), Add::add)
    }
}
