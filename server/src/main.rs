use std::env;
use std::io::Error;
use std::sync::Arc;
use di_ioc_lib::di::ioc_container::IocContainer;
use web_framework_lib::web::models::request::Request;
use web_framework_lib::web::server;
use web_framework_lib::web::models::response::Response;
use web_framework_lib::web::request_handling::route_handler_container::RouteHandlerContainer;
use web_framework_lib::web::util::enums::http_method_enum::HttpMethod;
use web_framework_lib::web::util::enums::mime_types::MimeTypes;
use web_framework_lib::web::util::parsers::json_parser::{JsonObject, JsonParseError};

/// It gets the second path cell from the request path, decodes it, and sets it as the response body
///
/// Arguments:
///
/// * `transaction`: &mut Transaction
pub fn path_param_test(req: &Request) -> Response {
    let path_cell: String = req
        .request_line_data()
        .get_path_cell_by_index_url_decoded(1)
        .expect("uwu");
    let mut res: Response = Response::ok();
    res.set_body(path_cell);
    res
}

pub fn json_test(req: &Request) -> Response {
    let body_as_json: Result<JsonObject, JsonParseError> = req.get_body_as_json();
    let body_as_json: JsonObject = body_as_json.unwrap();
    let mut res: Response = Response::ok();
    res.set_body(body_as_json.to_string());
    res.content_type(MimeTypes::JSON);
    res
}

pub fn index(_req: &Request) -> Response {
    let mut res: Response = Response::ok();
    let result: Result<(), Error> = res.set_body_to_file("/index.html");
    if let Err(_e) = result { res.set_status(404).set_reason_phrase("Not Found"); }
    res
}
/// It starts a server on port 7878 and registers the routes.
fn main() {
    static RUST_LOG: &str = "RUST_LOG";
    static DEBUG: &str = "debug";
    env::set_var(RUST_LOG, DEBUG);
    let mut container: IocContainer = IocContainer::default();
    let mut rhc: RouteHandlerContainer = RouteHandlerContainer::new();
    rhc.insert("/", index, HttpMethod::GET);
    rhc.insert("/hey/{a}/hey", path_param_test, HttpMethod::GET);
    rhc.insert("/json/test", json_test, HttpMethod::POST);
    container.install_reference_provider(Arc::new(rhc));
    server::start("7878", Arc::new(container));
}
