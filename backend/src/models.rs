use serde::{Serialize, Deserialize};
use super::schema::users;


#[derive(Default, Deserialize, Queryable, Debug)]
#[serde(default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

