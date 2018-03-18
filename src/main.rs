#[macro_use] extern crate rouille;

fn handler(request: &rouille::Request) -> rouille::Response
{
    rouille::match_assets(&request.remove_prefix("/static").unwrap(), "./src/front/")
}

fn main() {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) ["/static/index.html"] => { handler(&request) },
            (GET) ["/static/bundle.js"] => { handler(&request) },
            (GET) ["/"] => { rouille::Response::redirect_302("/static/index.html") },
            _ => rouille::Response::empty_404()
            )
        });
}