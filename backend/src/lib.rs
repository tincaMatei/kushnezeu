#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username,
        password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

