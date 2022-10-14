use std::str::FromStr;
use di_ioc_lib::di::ioc_container::IocContainer;
use std::sync::Arc;

use crate::web::models::transaction::response::Response;
use crate::web::models::transaction::Transaction;
use crate::web::request_handling::route_handler_container::RouteHandlerContainer;
use crate::web::util::enums::http_method_enum::HttpMethod;

//  █     █░▓█████   ▄████  ▄▄▄▄    ██▓     ▄▄▄      ▓█████▄
// ▓█░ █ ░█░▓█   ▀  ██▒ ▀█▒▓█████▄ ▓██▒    ▒████▄    ▒██▀ ██▌
// ▒█░ █ ░█ ▒███   ▒██░▄▄▄░▒██▒ ▄██▒██░    ▒██  ▀█▄  ░██   █▌
// ░█░ █ ░█ ▒▓█  ▄ ░▓█  ██▓▒██░█▀  ▒██░    ░██▄▄▄▄██ ░▓█▄   ▌
// ░░██▒██▓ ░▒████▒░▒▓███▀▒░▓█  ▀█▓░██████▒ ▓█   ▓██▒░▒████▓
// ░ ▓░▒ ▒  ░░ ▒░ ░ ░▒   ▒ ░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░ ▒▒▓  ▒
//   ▒ ░ ░   ░ ░  ░  ░   ░ ▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░ ░ ▒  ▒
//   ░   ░     ░   ░ ░   ░  ░    ░   ░ ░     ░   ▒    ░ ░  ░
//     ░       ░  ░      ░  ░          ░  ░      ░  ░   ░
//                               ░                    ░

/// It takes a `Transaction` and a `Container` and
/// calls the appropriate handler function for the request path
///
/// Arguments:
///
/// * `transaction`: The transaction object that is passed through the chain.
/// * `container`: Arc<Container> - This is the container that holds the route map.
pub fn enter_chain(mut transaction: Transaction, container: Arc<IocContainer>) {
    let path: &str = &transaction.req().request_line_data().path();

    // We get a reference to the container containing our mapped routes from the IocContainer.
    let route_map: &RouteHandlerContainer = container.get_ref()
        .expect("Failed to get RouteHandlerContainer.");

    let method: HttpMethod = HttpMethod::from_str(transaction.req().request_line_data().method())
        .expect("Invalid http method");

    //  Here we are matching the requested path to our mapped routes.
    if let Some(handler) = route_map.get_match(&path, &method) {
        handler(&mut transaction);
    } else { // We find no match, so we need to rule out static resources, or resolve.
        let mut was_static: bool = false;
        if transaction.req().request_line_data().method() == HttpMethod::GET.to_string() {
            was_static = rule_out_static_resources(&mut transaction);
        }
        if !was_static {
            let res: &mut Response = transaction.res_mut();
            res.set_status(404);
            res.set_reason_phrase("Not Found");
        }
    }
    match transaction.resolve() { // We're resolving the transaction, sending the response.
        Err(e) => { if log::log_enabled!(log::Level::Error) { log::error!("{}", e); } },
        Ok(_) => {
            if log::log_enabled!(log::Level::Info) {
                log::info!("Transaction resolved for {}, status: {}, path: {}",
                transaction.req().stream().peer_addr().unwrap(),
                transaction.res().status(),
                transaction.req().request_line_data().path());
            }
            if log::log_enabled!(log::Level::Debug) { log::debug!("\n{:?}", transaction); }
        }
    };
}

/// If the path contains a dot, then try to serve the file
///
/// Arguments:
///
/// * `transaction`: &mut Transaction
///
/// Returns:
///
/// A boolean value.
fn rule_out_static_resources(transaction: &mut Transaction) -> bool {
    let path: &str = &transaction.req().request_line_data().path().to_owned();
    // TODO: Look at this.
    if path.contains('.') {
        let res: &mut Response = transaction.res_mut();
        match res.set_body_to_file(&path) {
            Ok(_) => {
                res.set_status(200);
                res.set_reason_phrase("OK");
                return true
            },
            Err(e) => {
                if log::log_enabled!(log::Level::Error) {
                    log::error!("{}", e);
                }
            }
        }
    }
    false
}