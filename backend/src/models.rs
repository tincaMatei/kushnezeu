use serde::{Deserialize};
use super::schema::*;


#[derive(Default, Identifiable, Deserialize, Queryable, Debug)]
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

#[derive(Debug, Deserialize, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[table_name = "sessions"]
#[serde(default)]
pub struct Session<'a> {
    pub session_id: &'a str,
    pub user_id: i32,
    pub expire: chrono::NaiveDateTime,
}

#[derive(Debug, Default, Deserialize, Insertable, Queryable, Associations)]
#[table_name = "groups"]
#[serde(default)]
pub struct Group {
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable, Queryable)]
#[table_name = "content"]
#[serde(default)]
pub struct Content<'a> {
    pub groupname: &'a str,
    pub page: &'a str,
    pub contentbody: &'a str,
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

