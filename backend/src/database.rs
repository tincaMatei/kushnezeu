use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::result::ConnectionResult;
use diesel_migrations::*;

use super::*;

embed_migrations!("migrations");

#[derive(Clone)]
pub struct DatabaseServer {
    database_url: String,
}

impl DatabaseServer {
    pub fn start_database() -> DatabaseServer {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let db_connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
       
        DatabaseServer {
            database_url,
        }
    }

    pub fn connect(&self) -> ConnectionResult<PgConnection> {
        PgConnection::establish(&self.database_url)
    }

    pub fn run_migrations(&self) {
        embedded_migrations::run(&self.connect().unwrap());
    }
}
