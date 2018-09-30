use std::io::Read;
use diesel;
use diesel::sql_types;
use diesel::prelude::*;
use models::naive_date_form::NaiveDateForm;
use schema;
use schema::sales;
use handlers::base::Search;
use basic_model_actions;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Date,
                                                     sql_types::Nullable<sql_types::Integer>,
                                                     sql_types::Nullable<sql_types::Integer>,
                                                     sql_types::Nullable<sql_types::Text>),
                                                     schema::sales::table, diesel::pg::Pg>;

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Clone, Queryable, Debug, FromForm)]
pub struct Sale {
    pub id: i32,
    pub client_id: i32,
    pub sale_date: NaiveDateForm,
    pub sub_total: Option<i32>,
    pub total: Option<i32>,
    pub observation: Option<String>
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, FromForm)]
#[table_name="sales"]
pub struct NewSale {
    pub client_id: i32,
    pub sale_date: NaiveDateForm,
    pub sub_total: Option<i32>,
    pub total: Option<i32>,
    pub observation: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchSale {
    pub id: Option<i32>,
    pub client_id: Option<i32>,
    pub sale_date: Option<NaiveDateForm>,
    pub sub_total: Option<i32>,
    pub total: Option<i32>,
    pub observation: Option<String>
}

impl Sale {

    fn searching_records<'a>(search: Option<Search<SearchSale>>) -> BoxedQuery<'a> {
        use schema::sales::dsl::*;

        let mut query = schema::sales::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_sale) = search {
            let Search(sale) = search_sale;
            if let Some(sale_id) = sale.id {
                query = query.filter(id.eq(sale_id));
            }
            if let Some(sale_sale_date) = sale.sale_date {
                query = query.filter(sale_date.eq(sale_sale_date));
            }
            if let Some(sale_observation) = sale.observation {
                query = query.filter(observation.like(sale_observation));
            }
        }

        query
    }
}
basic_model_actions!(sales, Sale, NewSale, SearchSale);
from_data!(Sale);
from_data!(NewSale);
