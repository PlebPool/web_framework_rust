use std::env;
use std::sync::Arc;
use di_ioc_lib::di::ioc_container::IocContainer;
use web_framework_lib::web::server;
use web_framework_lib::web::models::transaction::response::Response;
use web_framework_lib::web::models::transaction::Transaction;
use web_framework_lib::web::request_handling::route_handler_container::RouteHandlerContainer;

pub fn index(transaction: &mut Transaction) {
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body_to_file("/html/index.html")
            .expect("Failed to read file");
}

pub fn path_param_test(transaction: &mut Transaction) {
    let path_cell: String = transaction.req()
        .request_line_data()
        .get_path_cell_by_index_url_decoded(1)
        .unwrap();
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body(path_cell);
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    let mut container: IocContainer = IocContainer::default();
    let mut rhc: RouteHandlerContainer = RouteHandlerContainer::new();
    rhc.insert("/", index);
    rhc.insert("/hey/{a}/hey", path_param_test);
    container.install_reference_provider(Arc::new(rhc));
    server::start("7878", Arc::new(container));
}
