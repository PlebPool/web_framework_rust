use std::collections::HashMap;
use std::sync::Arc;
use crate::application::di::container::Container;
use crate::web::server::data::models::transaction::Transaction;
use crate::web::server::HandlerFunction;


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
    let path = transaction.req().request_line_data().path.to_owned();
    let route_map: &HashMap<String, HandlerFunction> = container.get_ref()
        .expect("Failed to get route_map");
    if let Some(handler) = route_map.get(&path) {
        handler(&mut transaction);
    } else {
        let res = transaction.res_mut();
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