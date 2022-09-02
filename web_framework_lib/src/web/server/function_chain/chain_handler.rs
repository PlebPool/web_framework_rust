use std::sync::Arc;
use di_ioc_lib::di::ioc_container::IocContainer;
use crate::web::server::data::enums::http_method_enum::HttpMethod;
use crate::web::server::data::models::transaction::response::Response;
use crate::web::server::data::models::transaction::Transaction;
use crate::web::server::function_chain::route_handler_container::RouteHandlerContainer;

mod handlers {
    pub mod handler_config;
    pub mod static_resource_handler;
}

/// It takes a `Transaction` and a `Container` and
/// calls the appropriate handler function for the request path
///
/// Arguments:
///
/// * `transaction`: The transaction object that is passed through the chain.
/// * `container`: Arc<Container> - This is the container that holds the route map.
pub fn enter_chain(mut transaction: Transaction, container: Arc<IocContainer>) {
    let path: &str = &transaction.req().request_line_data().path();
    let route_map: &RouteHandlerContainer = container.get_ref()
        .expect("Failed to get RouteHandlerContainer.");
    if let Some(handler) = route_map.get(&path) {
        handler(&mut transaction);
    } else {
        if transaction.req().request_line_data().method() == HttpMethod::GET.to_string() {
            let was_static: bool = rule_out_static_resources(&mut transaction);
            if !was_static {
                let res: &mut Response = transaction.res_mut();
                res.set_status(404);
                res.set_reason_phrase("Not Found");
            }
        }
    }
    match transaction.resolve() {
        Err(e) => { dbg!(e); },
        Ok(_) => { dbg!(transaction); }
    };
}

fn rule_out_static_resources(transaction: &mut Transaction) -> bool {
    let path: &str = &transaction.req().request_line_data().path();
    if path.contains('.') {
        let res: &mut Response = transaction.res_mut();
        match res.set_body_to_file(&path) {
            Ok(_) => {
                res.set_status(200);
                res.set_reason_phrase("OK");
                return true
            },
            Err(e) => {
                dbg!(e);
            }
        }
    }
    false
}