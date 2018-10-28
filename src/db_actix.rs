use actix::prelude::*;

use actix_web::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use futures::Future;

use schema;
use models;

pub struct DBExecutor(pub PgConnection);

pub struct ShowPosts;

impl Message for ShowPosts {
    type Result = Result<Vec<models::Post>, String>;
}

impl<'a> Message for models::NewPost {
    type Result = Result<models::Post, String>;
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

impl<'a> Handler<models::NewPost> for DBExecutor {
    type Result = Result<models::Post, String>;

    fn handle(&mut self, new_post: models::NewPost, _: &mut Self::Context) -> Self::Result {
        use self::schema::posts::dsl::*;
        let conn = &self.0;
        diesel::insert_into(posts)
            .values(&new_post)
            .get_result(conn)
            .map_err(|_| "Could not insert new post.".to_string())

    }
}

impl Handler<ShowPosts> for DBExecutor {
    type Result = Result<Vec<models::Post>, String>;

    fn handle(&mut self, _msg: ShowPosts, _: &mut Self::Context) -> Self::Result {
        use self::schema::posts::dsl::*;
        let conn = &self.0;
        posts.load::<models::Post>(conn)
            .map_err(|_| "Couldn't load posts.".to_string())
    }
}
