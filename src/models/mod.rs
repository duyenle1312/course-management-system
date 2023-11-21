// models/mod.rs
use super::schema::courses;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub title: String,
    pub description: String,
    pub published: bool,
}