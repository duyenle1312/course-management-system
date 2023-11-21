extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, launch, post, routes};

mod models;
pub mod schema;

use rocket_dyn_templates::{context, Template};
use std::env;
pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct NewCourse {
    title: String,
    description: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/course", format = "json", data = "<course>")]
fn create_course(course: Json<NewCourse>) -> Result<Created<Json<NewCourse>>> {
    use self::schema::courses::dsl::*;
    use models::NewCourse;
    let mut connection = establish_connection_pg();

    let new_course = NewCourse {
        title: course.title.to_string(),
        description: course.description.to_string(),
        published: true,
    };

    diesel::insert_into(courses)
        .values(&new_course)
        .execute(&mut connection)
        .expect("Error saving new course");

    Ok(Created::new("/").body(course))
}

#[get("/courses")]
fn index() -> Template {
    use self::models::Course;

    let connection = &mut establish_connection_pg();
    let results = self::schema::courses::dsl::courses
        .load::<Course>(connection)
        .expect("Error loading courses");
    Template::render("courses", context! {courses: &results, count: results.len()})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_course])
        .mount("/", routes![index])
        .attach(Template::fairing())
}