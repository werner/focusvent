use std::io::Read;
use std::str::FromStr;
use diesel;
use diesel::sql_types;
use diesel::prelude::*;
use models::db_connection::*;
use models::naive_date_form::NaiveDateForm;
use models::sale_product::SaleProduct;
use models::sale_product::NewSaleProduct;
use models::calculation::Calculation;
use models::item_calculation::ItemCalculation;
use models::money::Money;
use models::sale_status::SaleStatus;
use models::sale_status::SaleStatusMapping;
use models::client::Client;
use models::client::BasicModelActions;
use rocket::{ Request, Data };
use rocket::data:: { FromData, Outcome };
use rocket::http::Status;
use rocket::Outcome::{ Failure, Success };
use schema;
use schema::sales;
use serde_json;
use handlers::base::Search;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Date,
                                                     sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Integer,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Integer,
                                                     SaleStatusMapping,
                                                     sql_types::Nullable<sql_types::Date>
                                                     ),
                                                     schema::sales::table, diesel::pg::Pg>;

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Clone, Queryable, Debug, FromForm)]
#[table_name="sales"]
pub struct Sale {
    pub id: i32,
    pub client_id: i32,
    pub sale_date: NaiveDateForm,
    pub sub_total: Money,
    pub sub_total_without_discount: Money,
    pub discount_calculated: Money,
    pub taxes_calculated: Money,
    pub total: Money,
    pub observation: Option<String>,
    pub currency_id: i32,
    #[serde(skip_deserializing)]
    pub status: SaleStatus,
    pub expiring_date: Option<NaiveDateForm>
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, FromForm)]
#[table_name="sales"]
pub struct NewSale {
    pub client_id: i32,
    pub sale_date: NaiveDateForm,
    pub sub_total: Option<Money>,
    pub sub_total_without_discount: Option<Money>,
    pub discount_calculated: Option<Money>,
    pub taxes_calculated: Option<Money>,
    pub total: Option<Money>,
    pub observation: Option<String>,
    pub currency_id: i32,
    #[serde(skip_deserializing)]
    pub status: SaleStatus,
    pub expiring_date: Option<NaiveDateForm>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullSale {
    pub sale: Sale,
    pub sale_products: Vec<SaleProduct>
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
    pub sub_total: Option<Money>,
    pub sub_total_without_discount: Option<Money>,
    pub discount_calculated: Option<Money>,
    pub taxes_calculated: Option<Money>,
    pub total: Option<Money>,
    pub observation: Option<String>,
    pub currency_id: Option<i32>,
    pub status: Option<SaleStatus>,
    pub expiring_date: Option<NaiveDateForm>
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

    pub fn show(request_id: i32) -> Result<FullSale, diesel::result::Error> {
        use schema::sales::dsl::*;
        use schema::sale_products;

        let connection = establish_connection();

        let sale_result = sales
            .find(request_id)
            .get_result::<Sale>(&connection)?;

        let sale_products_result = sale_products::dsl::sale_products
            .filter(sale_products::dsl::sale_id.eq(sale_result.id))
            .load::<SaleProduct>(&connection)?;

        Ok(FullSale {
            sale: sale_result,
            sale_products: sale_products_result
        })
    }
 
    pub fn create(full_new_sale: FullNewSale) -> Result<Sale, diesel::result::Error> {
        let connection = establish_connection();

        let sale: Result<Sale, diesel::result::Error> = diesel::insert_into(sales::table)
            .values(&full_new_sale.sale_with_calculations())
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
                  currency_id.eq(full_sale.sale.currency_id),
                  sale_date.eq(&full_sale.sale.sale_date),
                  observation.eq(&full_sale.sale.observation),
                  sub_total.eq(full_sale.calculate_sub_total()),
                  sub_total_without_discount.eq(full_sale.subtotal_without_discount()),
                  discount_calculated.eq(full_sale.calculate_discount()),
                  taxes_calculated.eq(full_sale.calculate_taxes()),
                  total.eq(full_sale.calculate_total())))
            .get_result::<Sale>(&connection);

        if let Ok(db_sale) = &sale {
            SaleProduct::batch_action(full_sale.sale_products, db_sale.id)?;
        }

        sale
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        use schema::sales::dsl::*;
        let connection = establish_connection();

        diesel::delete(sales.find(param_id))
            .execute(&connection)
    }

    pub fn client(&self) -> Result<Client, diesel::result::Error> {
        Client::show(self.client_id)
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
            if let Some(sale_status) = sale.status {
                query = query.filter(status.eq(sale_status));
            }
        }

        query
    }
}

impl FullNewSale {
    pub fn sale_with_calculations(&self) -> NewSale {
        let mut sale = self.sale.clone();
        sale.sub_total = Some(self.calculate_sub_total());
        sale.sub_total_without_discount = Some(self.subtotal_without_discount());
        sale.discount_calculated = Some(self.calculate_discount());
        sale.taxes_calculated = Some(self.calculate_taxes());
        sale.total = Some(self.calculate_total());
        sale
    }

    pub fn calculate_sub_total(&self) -> Money {
        let items = self.get_items();
        let calculation = Calculation::new(items);
        calculation.subtotal()
    }

    pub fn calculate_total(&self) -> Money {
        let items = self.get_items();
        let calculation = Calculation::new(items);
        calculation.calculate_total()
    }

    pub fn subtotal_without_discount(&self) -> Money {
        let items = self.get_items();
        let calculation = Calculation::new(items);
        calculation.subtotal_without_discount()
    }

    pub fn calculate_discount(&self) -> Money {
        let items = self.get_items();
        let calculation = Calculation::new(items);
        calculation.calculate_discount()
    }

    pub fn calculate_taxes(&self) -> Money {
        let items = self.get_items();
        let calculation = Calculation::new(items);
        calculation.calculate_taxes()
    }

    fn get_items(&self) -> Vec<ItemCalculation> {
         self
        .sale_products
        .iter()
        .map(|new_sale_product| new_sale_product.to_item_calc_method())
        .collect::<Vec<ItemCalculation>>()
    }
}

impl FromStr for Sale {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromStr for SearchSale {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromData for FullNewSale {
    type Error = String;

    fn from_data( _: &Request, data: Data) -> Outcome<Self, String> {

        let mut string_data = String::new();
        if let Err(e) = data.open().read_to_string(&mut string_data) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let maybe_full_new_sale = serde_json::from_str(&string_data);

        match maybe_full_new_sale {
            Ok(ok_full_new_sale) => {
                let full_new_sale: FullNewSale = ok_full_new_sale;
                if full_new_sale.sale_products.is_empty() {
                    return Failure(( Status::UnprocessableEntity, "No products selected!".to_string() ));
                }
                Success(full_new_sale)
            },
            Err(err) => {
                println!("Error deserializing {:#?} {:?}", &string_data, err);
                return Failure((Status::BadRequest, format!("Error deserializing {:?} {:?}", &string_data, err)))
            }
        }
    }
}
