#[macro_use] extern crate rouille;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate postgres;

use std::sync::Mutex;
use postgres::Connection;
use postgres::TlsMode;
use postgres::transaction::Transaction;
use rouille::Request;
use rouille::Response;

fn handler(request: &rouille::Request) -> rouille::Response
{
    rouille::match_assets(&request.remove_prefix("/static").unwrap(), "./src/front/")
}

fn main() {
    let db = {
        let db = Connection::connect("postgres://postgres:mysecretpassword@localhost:5432/test", TlsMode::None);
        Mutex::new(db.expect("Failed to connect to database"))
    };

    {
        let sql = "CREATE TABLE IF NOT EXISTS notes (
            id SERIAL PRIMARY KEY,
            content TEXT NOT NULL
        );";
        db.lock().unwrap().execute(sql, &[]).expect("Failed to initialize database");
    }

    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", move |request| {
        let db = db.lock().unwrap();

        let db = db.transaction().unwrap();

        let response = app_routes(&request, &db);

        if response.is_success() {
            db.commit().unwrap();
        }
        response
    });
}

fn app_routes(request: &Request, db: &Transaction) -> Response {
    router!(request,
        (GET) ["/static/index.html"] => { handler(&request) },
        (GET) ["/static/bundle.js"] => { handler(&request) },
        (GET) ["/"] => { rouille::Response::redirect_302("/static/index.html") },
        _ => api_routes(request, db)
    )
}

fn api_routes(request: &Request, db: &Transaction) -> Response {
    router!(request,
        (GET) (/api/data) => {get_data_handler(request, db)},
        (POST) (/api/data) => {post_data_handler(request, db)},
        _ => rouille::Response::empty_404()
    )
}
#[derive(Serialize, Deserialize)]
struct DataRequest {
    content: String
}

#[derive(Serialize, Deserialize)]
struct Data {
    id: i32,
    content: String
}

fn get_data_handler(request: &Request, db: &Transaction) -> Response {
    let mut out = Vec::new();
    // We perform the query and iterate over the rows, writing each row to `out`.
    for row in &db.query("SELECT id, content FROM notes", &[]).unwrap() {
        let id: i32 = row.get(0);
        let content: String = row.get(1);
        out.push(Data { id: id, content: content });
    }

    Response::json(&out)

}


fn post_data_handler(request: &Request, db: &Transaction) -> Response {
    let data_request: DataRequest = try_or_400!(rouille::input::json_input::<DataRequest>(&request));
    let save_result = save_data(&data_request, db);

    let mut response = Response::json(&save_result);
    response.status_code = 201;
    response
}

fn save_data(dr: &DataRequest, db: &Transaction) -> Data {
    let mut id: Option<i32> = None;
    for row in &db.query("INSERT INTO notes(content) VALUES ($1) RETURNING id", &[&dr.content]).unwrap() {
        id = Some(row.get(0));
    }

    let id = id.unwrap();
    let data = Data {id : id, content : dr.content.to_string()};
    data
}
