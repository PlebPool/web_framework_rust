use std::sync::Arc;
use di_ioc_lib::di::container::Container;
use crate::web::server::data::models::transaction::response::Response;
use crate::web::server::data::models::transaction::Transaction;
use crate::web::server::function_chain::route_handler_container::RouteHandlerContainer;

// TODO: Impl chain.
// TODO: Build chain.
// TODO: Execute chain.
// TODO: Resolve transaction.
// TODO: Log transaction to terminal.

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
pub fn enter_chain(mut transaction: Transaction, container: Arc<Container>) {
    let path: String = transaction.req().request_line_data().path.to_owned();

    let route_map: &RouteHandlerContainer = container.get_ref()
        .expect("Failed to get RouteHandlerContainer.");

    if let Some(handler) = route_map.get(&path) {
        handler(&mut transaction);
    } else {
        if transaction.req().request_line_data().path.contains('.') {
            // TODO: Check if extension is valid.
            // TODO: Check if req is looking in valid directory.
            // TODO: Send file if exists.
        }
        let res: &mut Response = transaction.res_mut();
        res.set_status(404);
        res.set_reason_phrase("Not Found");
    }
    match transaction.resolve() {
        Err(e) => {
            println!("{}", e);
        },
        Ok(_) => {
            println!("Request handled: {:#?}", transaction);
        }
    };
}