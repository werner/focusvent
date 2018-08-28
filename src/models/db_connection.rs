use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url_env_var_name = if cfg!(test) {
        "DATABASE_URL"
    } else {
        "TEST_DATABASE_URL"
    };

    let database_url = env::var(database_url_env_var_name)
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}