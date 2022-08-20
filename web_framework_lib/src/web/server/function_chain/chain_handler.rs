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

pub fn enter_chain(mut transaction: Transaction, container: Arc<Container>) {
    let path = transaction.req().request_line_data().path.to_owned();
    if path == "/" {
        let handler: &HashMap<String, HandlerFunction> = container.get_ref().unwrap();
        handler.get("/").unwrap()(&mut transaction);
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