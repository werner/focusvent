use basic_model_actions;
use diesel;
use diesel::prelude::*;
use diesel::sql_types;
use handlers::base::Search;
use models::db_connection;
use schema;
use schema::currencies;
use std::io::Read;

type BoxedQuery<'a> = diesel::query_builder::BoxedSelectStatement<
    'a,
    (
        sql_types::Integer,
        sql_types::Text,
        sql_types::Text,
        sql_types::Text,
        sql_types::Bool,
        sql_types::Bool,
    ),
    schema::currencies::table,
    diesel::pg::Pg,
>;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, FromForm)]
#[table_name = "currencies"]
pub struct Currency {
    pub id: i32,
    pub value: String,
    pub symbol: String,
    pub decimal_point: String,
    pub default_currency: bool,
    pub in_use: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchCurrency {
    id: Option<i32>,
    value: Option<String>,
    symbol: Option<String>,
    decimal_point: Option<String>,
    default_currency: Option<bool>,
    in_use: Option<bool>,
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name = "currencies"]
pub struct NewCurrency {
    value: String,
    symbol: String,
    decimal_point: String,
    default_currency: bool,
    in_use: bool,
}

impl Currency {
    pub fn get_currency() -> Self {
        use schema::currencies::dsl::*;
        let connection = db_connection::establish_connection();

        let maybe_currency = currencies
            .filter(in_use.eq(true))
            .get_result::<Currency>(&connection);

        match maybe_currency {
            Ok(currency) => currency,
            Err(_) => Self::get_default_currency()
        }

    }

    fn get_default_currency() -> Self {
        use schema::currencies::dsl::*;
        let connection = db_connection::establish_connection();

        let default = currencies
            .filter(default_currency.eq(true))
            .get_result::<Currency>(&connection);

        match default {
            Ok(currency) => currency,
            Err(_) => Currency {
                id: 0,
                value: "USD".to_string(),
                symbol: "$".to_string(),
                decimal_point: ".".to_string(),
                default_currency: true,
                in_use: true
            },
        }
    }

    fn searching_records<'a>(search: Option<Search<SearchCurrency>>) -> BoxedQuery<'a> {
        use schema::currencies::dsl::*;

        let mut query = schema::currencies::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_currencies) = search {
            let Search(currency) = search_currencies;
            if let Some(currency_id) = currency.id {
                query = query.filter(id.eq(currency_id));
            }
            if let Some(currency_value) = currency.value {
                query = query.filter(value.like(currency_value));
            }
            if let Some(currency_decimal_point) = currency.decimal_point {
                query = query.filter(decimal_point.like(currency_decimal_point));
            }
        }

        query
    }
}

basic_model_actions!(currencies, Currency, NewCurrency, SearchCurrency);
from_data!(Currency);
from_data!(NewCurrency);
