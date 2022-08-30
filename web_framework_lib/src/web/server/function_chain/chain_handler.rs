use std::sync::Arc;
use di_ioc_lib::di::container::Container;
use crate::web::server::data::enums::static_file_ext_enum::StaticFileExt;
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
        if transaction.req().request_line_data().method() == "GET" {
            let was_static: bool = rule_out_static_resources(&mut transaction);
            if !was_static {
                let res: &mut Response = transaction.res_mut();
                res.set_status(404);
                res.set_reason_phrase("Not Found");
            }
        }
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

fn rule_out_static_resources(transaction: &mut Transaction) -> bool {
    let path: String = transaction.req().request_line_data().path().to_owned();
    if path.contains('.') {
        if let Ok(ext) = StaticFileExt
        ::from_str(path.split_once('.').expect("Split failed").1) {
            let res: &mut Response = transaction.res_mut();
            match res.set_body_to_file(&path) {
                Ok(_) => {
                    res.set_status(200);
                    res.set_reason_phrase("OK");
                    res.add_header("Content-Type", ext.mime_type()
                        .expect("Failed to get mime type").to_string());
                    return true
                },
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            }
        }
    }
    false

}

// if path.contains('.') {
// let split = path.split_once('.').expect("Split failed");
// let ext: Result<StaticFileExt, ()> = StaticFileExt::from_str(split.1);
// if let Ok(sfe) = ext {
// let res: &mut Response = transaction.res_mut();
// res.set_body_to_file(path.as_str())
// .expect("Failed to set file to body");
// res.add_header("Content-Type", sfe.mime_type()
// .expect("Failed to get mime type").to_string());
// res.set_status(200);
// res.set_reason_phrase("OK");
// }
// } else {
// let res: &mut Response = transaction.res_mut();
// res.set_status(404);
// res.set_reason_phrase("Not Found");
// }