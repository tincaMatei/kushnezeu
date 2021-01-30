use serde::{Deserialize};
use super::schema::*;


#[derive(Default, Identifiable, Deserialize, Queryable, Debug)]
#[serde(default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Debug, Default, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[table_name = "sessions"]
#[serde(default)]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    pub expire: chrono::NaiveDateTime,
}

impl Default for Session {
    fn default() -> Self { 
        Self {
            session_id: String::new(),
            user_id: 0,
            expire: chrono::NaiveDateTime::from_timestamp(0, 0), 
        }
    }
}

#[derive(Debug, Default, Deserialize, Insertable, Queryable, Associations)]
#[table_name = "groups"]
#[serde(default)]
pub struct Group {
    pub name: String,
}

#[derive(Debug, Default, Deserialize, Insertable, Queryable)]
#[table_name = "content"]
#[serde(default)]
pub struct Content {
    pub groupname: String,
    pub page: String,
    pub contentbody: Option<String>,
}

#[derive(Debug, Default, Deserialize, Insertable, Queryable)]
#[table_name = "privillege"]
#[serde(default)]
pub struct Privillege {
    pub user_id: i32,
    pub groupname: String,
    pub rights: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrivillegeByUsername {
    pub username: String,
    pub groupname: String,
    pub rights: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct SessionContentPost {
    pub session_id: String,
    pub content: String,
}

