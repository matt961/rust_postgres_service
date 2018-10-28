extern crate rust_postgres;
extern crate diesel;

use self::rust_postgres::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_postgres::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
