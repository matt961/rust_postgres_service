#[macro_use]
extern crate diesel;

extern crate dotenv;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;

pub mod schema;
pub mod models;
pub mod db_actix;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title.to_string(),
        body: body.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
