use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: u32
}

#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: &'a u32
}
