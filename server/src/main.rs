use std::env;
use std::sync::Arc;
use di_ioc_lib::di::ioc_container::IocContainer;
use web_framework_lib::web::server;
use web_framework_lib::web::models::transaction::response::Response;
use web_framework_lib::web::models::transaction::Transaction;
use web_framework_lib::web::request_handling::route_handler_container::RouteHandlerContainer;
use web_framework_lib::web::util::enums::http_method_enum::HttpMethod;

/// `index` is a function that takes a mutable reference to a `Transaction` and returns nothing
///
/// Arguments:
///
/// * `transaction`: &mut Transaction
pub fn index(transaction: &mut Transaction) {
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body_to_file("/index.html")
            .expect("Failed to read file.");
}

/// It gets the second path cell from the request path, decodes it, and sets it as the response body
///
/// Arguments:
///
/// * `transaction`: &mut Transaction
pub fn path_param_test(transaction: &mut Transaction) {
    let path_cell: String = transaction.req()
        .request_line_data()
        .get_path_cell_by_index_url_decoded(1)
            .expect("Failed to get url path cell.");
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body(path_cell);
}

static RUST_LOG: &str = "RUST_LOG";
static DEBUG: &str = "debug";

/// It starts a server on port 7878 and registers the routes.
fn main() {
    env::set_var(RUST_LOG, DEBUG);
    let mut container: IocContainer = IocContainer::default();
    let mut rhc: RouteHandlerContainer = RouteHandlerContainer::new();
    rhc.insert(
        "/",
        index,
        HttpMethod::GET
    );
    rhc.insert(
        "/hey/{a}/hey",
        path_param_test,
        HttpMethod::GET
    );
    container.install_reference_provider(Arc::new(rhc));
    server::start("7878", Arc::new(container));
}
