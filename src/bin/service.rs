extern crate rust_postgres;
extern crate diesel;
extern crate actix_web;
extern crate actix;
extern crate futures;

use futures::prelude::*;

use self::rust_postgres::*;
use self::diesel::prelude::*;

use self::rust_postgres::db_actix;

use actix_web::{
    server, App, HttpRequest, HttpResponse, AsyncResponder, http::Method, FutureResponse, HttpMessage
};

use actix::prelude::*;

struct Pg {
    client: Addr<db_actix::DBExecutor>
}

fn show_posts(req: &HttpRequest<Pg>) -> FutureResponse<HttpResponse> {
    req.state().client.send(db_actix::ShowPosts)
        .from_err()
        .and_then(|res| match res {
            Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
            Err(_) => Ok(HttpResponse::InternalServerError().into())
        }).responder()
}

fn new_post(req: &HttpRequest<Pg>) -> FutureResponse<HttpResponse> {
    let db_client = req.state().client.clone();
    req.clone().json().from_err()
        .map(|new_post: models::NewPost| {
            println!("new post: {:?}", new_post);
            new_post
        })
        .and_then(move |new_post| {
            db_client.send(new_post.clone())
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(new_post) => Ok(HttpResponse::Ok().json(new_post)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into())
                    }
                })
        }).responder()
}

fn main() {
    let sys = actix::System::new("rust_postgres");

    let addr = SyncArbiter::start(2, || {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        db_actix::DBExecutor(PgConnection::establish(&db_url).unwrap())
    });

    const LISTEN_IP: &'static str = "0.0.0.0:9000";

    let s = server::new(move || {
        App::with_state(Pg{client: addr.clone()})
            .resource("/", |r| r.method(Method::GET).a(show_posts))
            .resource("/new", |r| r.method(Method::POST).a(new_post))
    }).bind(LISTEN_IP).unwrap();
    s.start();

    println!("connecting to DB at `{}`", std::env::var("DATABASE_URL").unwrap());
    println!("starting server on `{}`", LISTEN_IP);
    sys.run();
}
