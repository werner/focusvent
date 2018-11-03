use diesel;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;
use diesel::sql_types;
use schema;
use schema::clients;
use handlers::base::Search;
use basic_model_actions;

type BoxedQuery<'a> = 
    diesel::query_builder::BoxedSelectStatement<'a, (sql_types::Integer,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>,
                                                     sql_types::Nullable<sql_types::Text>),
                                                     schema::clients::table, diesel::pg::Pg>;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, AsChangeset, FromForm)]
pub struct Client {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, FromForm)]
pub struct SearchClient {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="clients"]
pub struct NewClient {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

impl Client {

    fn searching_records<'a>(search: Option<Search<SearchClient>>) -> BoxedQuery<'a> {
        use schema::clients::dsl::*;

        let mut query = schema::clients::table.into_boxed::<diesel::pg::Pg>();

        if let Some(search_clients) = search {
            let Search(client) = search_clients;
            if let Some(clients_first_name) = client.first_name {
                query = query.filter(first_name.like(clients_first_name));
            }
            if let Some(clients_last_name) = client.last_name {
                query = query.filter(last_name.like(clients_last_name));
            }
            if let Some(clients_id) = client.id {
                query = query.filter(id.eq(clients_id));
            }
            if let Some(clients_company_name) = client.company_name {
                query = query.filter(company_name.like(clients_company_name));
            }
            if let Some(clients_email) = client.email {
                query = query.filter(email.like(clients_email));
            }
            if let Some(clients_phone) = client.phone {
                query = query.filter(phone.like(clients_phone));
            }
        }

        query
    }
}

basic_model_actions!(clients, Client, NewClient, SearchClient);
