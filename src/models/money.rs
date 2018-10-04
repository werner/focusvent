use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess};

#[allow(dead_code)]
pub struct Money {
    value: i32,
    currency: String,
    decimal_point: String
}

impl Money {
    fn new(value: i32, currency: String, decimal_point: String) -> Self {
        Money { value, currency, decimal_point }
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Value, Currency, DecimalPoint };

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
                            "decimal_point" => Ok(Field::DecimalPoint),
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
                let value: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let currency: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let decimal_point: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let formatted_value: String = value
                                                  .chars()
                                                  .filter(|&c| !decimal_point.contains(c))
                                                  .collect();
                let parsed_value = formatted_value.parse::<i32>()
                    .map_err(|_val| de::Error::unknown_variant(&value, &["value"]))?;

                Ok(Money::new(parsed_value, currency, decimal_point))
            }

        }

        const FIELDS: &'static [&'static str] = &["value", "currency", "decimal_point"];
        deserializer.deserialize_struct("Money", FIELDS, MoneyVisitor)
    }
}