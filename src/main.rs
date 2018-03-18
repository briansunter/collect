#[macro_use] extern crate rouille;

fn main() {

    rouille::start_server("localhost:8000", move |request|
        rouille::match_assets(&request, "./src/front/")
    );

}