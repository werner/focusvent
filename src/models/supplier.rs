use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sql_types;
use diesel::TextExpressionMethods;
use diesel::ExpressionMethods;
use schema;
use schema::suppliers;
use handlers::base::Search;
use basic_model_actions;

type BoxedQuery<'a> =
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer, 
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Text,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>),
                                                     schema::suppliers::table, diesel::pg::Pg>;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, AsChangeset, FromForm)]
pub struct Supplier {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: String,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, FromForm)]
pub struct SearchSupplier {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="suppliers"]
pub struct NewSupplier {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: String,
    pub email: Option<String>,
    pub phone: Option<String>
}

impl Supplier {

    fn searching_records<'a>(search: Option<Search<SearchSupplier>>) -> BoxedQuery<'a> {
        use schema::suppliers::dsl::*;

        let mut query = schema::suppliers::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_suppliers) = search {
            let Search(supplier) = search_suppliers;
            if let Some(suppliers_first_name) = supplier.first_name {
                query = query.filter(first_name.like(suppliers_first_name));
            }
            if let Some(suppliers_last_name) = supplier.last_name {
                query = query.filter(last_name.like(suppliers_last_name));
            }
            if let Some(suppliers_id) = supplier.id {
                query = query.filter(id.eq(suppliers_id));
            }
            if let Some(suppliers_company_name) = supplier.company_name {
                query = query.filter(company_name.like(suppliers_company_name));
            }
            if let Some(suppliers_email) = supplier.email {
                query = query.filter(email.like(suppliers_email));
            }
            if let Some(suppliers_phone) = supplier.phone {
                query = query.filter(phone.like(suppliers_phone));
            }
        }

        query
    }
}

basic_model_actions!(suppliers, Supplier, NewSupplier, SearchSupplier);
from_data!(Supplier);
from_data!(NewSupplier);
