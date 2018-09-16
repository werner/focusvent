use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::clients;
use schema::clients::dsl::*;
use taxonomy;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, AsChangeset)]
pub struct Client {
    pub id: i32,
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

taxonomy!(clients, Client, NewClient);
from_data!(Client);
from_data!(NewClient);
