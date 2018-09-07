use std::io::Read;
use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use schema::clients;
use schema::clients::dsl::*;
use models::db_connection;

#[derive(Serialize, Deserialize, Clone, Queryable, Debug)]
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

impl Client {

    pub fn list(limit: i64, offset: i64) -> Result<Vec<Client>, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        clients
            .limit(limit)
            .offset(offset)
            .load::<Client>(&connection)
    }

    pub fn create(new_client: NewClient) -> Result<Client, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::insert_into(clients::table)
            .values(&new_client)
            .get_result(&connection)
    }

    pub fn show(request_id: i32) -> Result<Client, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        clients
            .find(request_id)
            .get_result::<Client>(&connection)
    }

    pub fn update(param_id: i32, client: Client) -> Result<Client, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::update(clients.find(param_id))
            .set((first_name.eq(client.first_name),
                  last_name.eq(client.last_name),
                  company_name.eq(client.company_name),
                  phone.eq(client.phone),
                  email.eq(client.email)))
            .get_result::<Client>(&connection)
    }

    pub fn delete(param_id: i32) -> Result<usize, diesel::result::Error> {
        let connection = db_connection::establish_connection();

        diesel::delete(clients.find(param_id))
            .execute(&connection)
    }
}

from_data!(Client);
from_data!(NewClient);