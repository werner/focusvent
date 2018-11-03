use diesel;
use diesel::prelude::*;
use diesel::sql_types;
use schema;
use schema::prices;
use handlers::base::Search;
use basic_model_actions;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer, sql_types::Text),
                                                     schema::prices::table, diesel::pg::Pg>;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash, Debug, Clone, AsChangeset, FromForm)]
pub struct Price {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchPrice {
    pub id: Option<i32>,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="prices"]
pub struct NewPrice {
    pub name: String
}

impl Price {

    fn searching_records<'a>(search: Option<Search<SearchPrice>>) -> BoxedQuery<'a> {
        use schema::prices::dsl::*;

        let mut query = schema::prices::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_price) = search {
            let Search(price) = search_price;
            if let Some(price_name) = price.name {
                query = query.filter(name.like(price_name));
            }
            if let Some(price_id) = price.id {
                query = query.filter(id.eq(price_id));
            }
        }

        query
    }
}
 
basic_model_actions!(prices, Price, NewPrice, SearchPrice);
