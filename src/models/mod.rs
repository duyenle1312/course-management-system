// models/mod.rs
use super::schema::posts;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}