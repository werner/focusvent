use std::{ fmt, str };
use std::ops::Deref;
use std::num::ParseFloatError;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, Unexpected};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use diesel::expression::{ AppearsOnTable, Expression, NonAggregate };
use diesel::query_builder::{ AstPass, QueryFragment };
use diesel::result::QueryResult;
use diesel::sql_types::Integer;
use diesel::Queryable;
use diesel::pg::Pg;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use models::currency::Currency;

#[derive(Clone, Debug)]
pub struct Money {
    value: i32,
    currency: Currency
}

impl Money {
    fn new(value: i32, currency: Currency) -> Self {
        Money { value, currency }
    }

    fn to_i32(currency: &Currency, value: &str) -> Result<i32, ParseFloatError> {
        let replaced_value = value.replace(&currency.decimal_point, ".");
        let float_value = replaced_value.parse::<f64>()?;
        Ok((float_value * 100.0).round() as i32)
    }

    fn to_f64_string(&self) -> String {
        let float_value = (self.value  as f64) / 100.0;
        let string_value = format!("{}", float_value);
        string_value.replace(".", &self.currency.decimal_point)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Value, Currency };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value` or `currency`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "value" => Ok(Field::Value),
                            "currency" => Ok(Field::Currency),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MoneyVisitor;

        impl<'de> Visitor<'de> for MoneyVisitor {
            type Value = Money;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Money")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Money, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value: &str = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let currency: Currency = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let parsed_value = Money::to_i32(&currency, value)
                    .map_err(|_val| de::Error::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Money::new(parsed_value, currency))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let currency = Currency::get_default_currency();

                let parsed_value = Money::to_i32(&currency, value)
                    .map_err(|_val| de::Error::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Money::new(parsed_value, currency))
            }

        }

        const FIELDS: &'static [&'static str] = &["value", "currency"];
        deserializer.deserialize_struct("Money", FIELDS, MoneyVisitor)
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Money", 2)?;

        state.serialize_field("value", &self.to_f64_string())?;
        state.serialize_field("currency", &self.currency)?;
        state.end()
    }
}

impl Expression for Money {
    type SqlType = Integer;
}

impl<QS> AppearsOnTable<QS> for Money {}

impl<'v> FromFormValue<'v> for Money {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Money, &'v RawStr> {
        match form_value.parse() {
            Ok(money) => Ok(money),
            _ => Err(form_value),
        }
    }
}

impl Deref for Money{
    type Target = i32;

    fn deref(&self)-> &i32 {
        &self.value
    }
}

impl Queryable<Integer, Pg> for Money {
    type Row = i32;

    fn build(row: Self::Row) -> Self {
        Money {
            value: row,
            currency: Currency::get_default_currency()
        }
    }
}

impl QueryFragment<Pg> for Money {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" INTEGER");
        Ok(())
    }
}
