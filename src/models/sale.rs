use std::io::Read;
use diesel;
use diesel::sql_types;
use diesel::prelude::*;
use models::db_connection::*;
use models::naive_date_form::NaiveDateForm;
use models::sale_product::SaleProduct;
use models::sale_product::NewSaleProduct;
use schema;
use schema::sales;
use handlers::base::Search;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullNewSale {
    sale: NewSale,
    sale_products: Vec<NewSaleProduct>
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
    pub fn list(limit: i64, offset: i64, search: Option<Search<SearchSale>>) ->
        Result<Vec<Sale>, diesel::result::Error> {
            let connection = establish_connection();
            
            let query = Self::searching_records(search);

            query
                .limit(limit)
                .offset(offset)
                .load(&connection)
    }


    pub fn create(full_new_sale: FullNewSale) -> Result<Sale, diesel::result::Error> {
        let connection = establish_connection();

        let sale: Result<Sale, diesel::result::Error> = diesel::insert_into(sales::table)
            .values(&full_new_sale.sale)
            .get_result(&connection);

        if let Ok(db_sale) = &sale {
            SaleProduct::batch_action(full_new_sale.sale_products, db_sale.id)?;
        }

        sale
    }

    pub fn update(param_id: i32, full_sale: FullNewSale) -> Result<Sale, diesel::result::Error> {
        use schema::sales::dsl::*;
        let connection = establish_connection();

        let sale = diesel::update(sales.find(param_id))
            .set((client_id.eq(full_sale.sale.client_id),
                  sale_date.eq(&full_sale.sale.sale_date),
                  observation.eq(&full_sale.sale.observation),
                  sub_total.eq(full_sale.sale.calculate_sub_total()),
                  total.eq(full_sale.sale.calculate_total())))
            .get_result::<Sale>(&connection);

        if let Ok(db_sale) = &sale {
            SaleProduct::batch_action(full_sale.sale_products, db_sale.id)?;
        }

        sale
    }

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

impl NewSale {
    pub fn calculate_sub_total(&self) -> Option<i32> {
        unimplemented!()
    }
    pub fn calculate_total(&self) -> Option<i32> {
        unimplemented!()
    }
}

from_data!(Sale);
from_data!(NewSale);
