use diesel;
use diesel::prelude::*;
use diesel::sql_types;
use crate::schema;
use crate::schema::costs;
use crate::handlers::base::Search;
use crate::basic_model_actions;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer, sql_types::Text),
                                                     schema::costs::table, diesel::pg::Pg>;

#[derive(Serialize, Deserialize, Queryable, Eq, PartialEq, Hash,
         Debug, AsChangeset, FromForm)]
pub struct Cost {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SearchCost {
    pub id: Option<i32>,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Eq, PartialEq, Hash, Debug)]
#[table_name="costs"]
pub struct NewCost {
    pub name: String
}
 
impl Cost {

    fn searching_records<'a>(search: Option<Search<SearchCost>>) -> BoxedQuery<'a> {
        use crate::schema::costs::dsl::*;

        let mut query = schema::costs::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_cost) = search {
            let Search(cost) = search_cost;
            if let Some(cost_name) = cost.name {
                query = query.filter(name.like(cost_name));
            }
            if let Some(cost_id) = cost.id {
                query = query.filter(id.eq(cost_id));
            }
        }

        query
    }
}

basic_model_actions!(costs, Cost, NewCost, SearchCost);
