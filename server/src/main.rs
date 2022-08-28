use std::collections::HashMap;
use std::sync::Arc;
use di_ioc_lib::di::container::Container;
use web_framework_lib::web::server;
use web_framework_lib::web::server::data::models::transaction::response::Response;
use web_framework_lib::web::server::data::models::transaction::Transaction;
use web_framework_lib::web::server::HandlerFunction;

pub fn index(transaction: &mut Transaction){
    let res: &mut Response = transaction.res_mut();
    res.set_status(200);
    res.set_reason_phrase("OK");
    res.set_body_to_file("/index.html").expect("Failed to read file");
    res.add_header("Content-Type", "text/html".to_string());
}

fn main() {
    let mut container = Container::default();
    let mut map: HashMap<String, HandlerFunction> = HashMap::new();
    map.insert("/".to_string(), index);
    container.install_reference_provider(Arc::new(map));
    server::start("7878", Arc::new(container))
}
