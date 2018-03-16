#[macro_use] extern crate rouille;
use std::io::Read;

fn main() {
    rouille::start_server("localhost:8000", move |request| router!(request,
       (GET) (/) => {
            let mut file = std::fs::File::open("./src/front/index.html").expect("Can't open file.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Can't read file");
            rouille::Response::html(contents)
         },
        _ => rouille::Response::empty_404()
    ));
}