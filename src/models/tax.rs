use std::io::Read;
use diesel;
use diesel::prelude::*;
use diesel::sql_types;
use schema;
use schema::taxes;
use handlers::base::Search;
use basic_model_actions;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer, sql_types::Text, sql_types::Integer),
                                                     schema::taxes::table, diesel::pg::Pg>;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, Clone, AsChangeset, FromForm)]
#[table_name="taxes"]
pub struct Tax {
    pub id: i32,
    pub name: String,
    pub percentage: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchTax {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub percentage: Option<i32>
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="taxes"]
pub struct NewTax {
    pub name: String,
    pub percentage: i32
}

impl Tax {

    fn searching_records<'a>(search: Option<Search<SearchTax>>) -> BoxedQuery<'a> {
        use schema::taxes::dsl::*;

        let mut query = schema::taxes::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_taxes) = search {
            let Search(tax) = search_taxes;
            if let Some(taxes_name) = tax.name {
                query = query.filter(name.like(taxes_name));
            }
            if let Some(taxes_id) = tax.id {
                query = query.filter(id.eq(taxes_id));
            }
            if let Some(taxes_percentage) = tax.percentage {
                query = query.filter(percentage.eq(taxes_percentage));
            }
        }

        query
    }
}

basic_model_actions!(taxes, Tax, NewTax, SearchTax);
from_data!(Tax);
from_data!(NewTax);
