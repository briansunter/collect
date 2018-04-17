mod data;
mod schema;
mod data_dao;
mod data_controller;
use data_controller::*;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rouille;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use std::env;

fn handler(request: &rouille::Request) -> rouille::Response
{
    rouille::match_assets(&request.remove_prefix("/static").unwrap(), "./src/front/")
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    println!("Now listening on localhost:8000");
    rouille::start_server("localhost:8000", move |request| {
        let conn = pool.get().unwrap();
        let dd = data_dao::DataDaoPG::new(&conn);
        return router!(request,
            (GET) ["/static/index.html"] => { handler(&request) },
            (GET) ["/static/bundle.js"] => { handler(&request) },
            (GET) ["/"] => { rouille::Response::redirect_302("/static/index.html") },
            (GET) ["/api/data"] => {get_data_handler(request, &dd)},
            (POST) ["/api/data"] => {post_data_handler(request, &dd)},
            _ => rouille::Response::empty_404()
        )
    });
}
