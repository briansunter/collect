use data_dao::*;
use rouille::{Request,Response};
use rouille;

#[derive(Serialize, Deserialize)]
struct DataRequest {
    content: String
}

pub fn get_data_handler(request: &Request, data_dao: &DataDao) -> Response {
    let out = data_dao.get_data();
    Response::json(&out)
}

pub fn post_data_handler(request: &Request, data_dao: &DataDao) -> Response {
    let data_request: DataRequest = try_or_400!(rouille::input::json_input::<DataRequest>(&request));
    let result = data_dao.create_data(&data_request.content);

    let mut response = Response::json(&result);
    response.status_code = 201;
    response
}
