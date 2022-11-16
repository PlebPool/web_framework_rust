use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;
use di_ioc_lib::di::ioc_container::IocContainer;
use std::sync::Arc;
use std::time::Instant;
use crate::web::models::request::Request;

use crate::web::models::response::Response;
use crate::web::request_handling::route_handler_container::RouteHandlerContainer;
use crate::web::util::enums::http_method_enum::HttpMethod;
use crate::web::util::parsers::request_parser;
use crate::web::util::parsers::request_parser::RequestParseError;

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

#[derive(Debug)]
pub enum HandleError {
    AlreadyResolved,
    HttpStatusNotSet,
    ResolveFailed,
    UnobtainedMutex
}

/// It takes a `Transaction` and a `Container` and
/// calls the appropriate handler function for the request path
///
/// Arguments:
///
/// * `transaction`: The transaction object that is passed through the chain.
/// * `container`: Arc<Container> - This is the container that holds the route map.
pub fn handle<'a>(tcp_stream: std::io::Result<TcpStream>, container: Arc<IocContainer>) -> bool {
    let mut unwrapped_stream: TcpStream = tcp_stream.unwrap(); // TODO.

    let req: Result<Request, RequestParseError> = request_parser::parse_request(
        unwrapped_stream.try_clone().unwrap(),
        [0; 1024]
    );

    if req.is_err() {
        let _ = unwrapped_stream.write(
            Response::new(500, "Internal Server Error")
                .get_as_u8_vec()
                .as_slice()
        );
        return false;
    }

    let req: Request = req.unwrap();

    if log::log_enabled!(log::Level::Info) {
        log::info!("Request Received from {}", req.stream().peer_addr().unwrap());
    }

    let start_time: Instant = Instant::now();
    let path: &str = req.request_line_data().path();

    // We get a reference to the container containing our mapped routes from the IocContainer.
    let route_map: &RouteHandlerContainer = container.get_ref()
        .expect("Failed to get RouteHandlerContainer.");

    let method: HttpMethod = HttpMethod::from_str(req.request_line_data().method())
        .expect("Invalid http method");

    //  Here we are matching the requested path to our mapped routes.
    let res: Response = if let Some(handler) = route_map.get_match(&path, &method) {
        handler(&req)
    } else { // We find no match, so we need to rule out static resources, or resolve.
        if req.request_line_data().method() == HttpMethod::GET.to_string() {
            let path_bind = req.request_line_data().path().to_owned();
            rule_out_static_resources(path_bind)
        } else {
            Response::not_found()
        }
    };
    let debug_res: Response = res.clone();
    match req.resolve(res) {
        Err(e) => {
            if log::log_enabled!(log::Level::Error) {
                log::error!("{:?}", e);
            }
        },
        Ok(_) => {
            let now: Instant = Instant::now();
            if log::log_enabled!(log::Level::Info) {
                log::info!("Transaction resolved for: {}, status: {}, path: {}, in: {}ms",
                    req.stream().peer_addr().unwrap(),
                    debug_res.status(),
                    req.request_line_data().path(),
                    now.duration_since(start_time).as_secs_f32() * 1000.0
                );
            }
            if log::log_enabled!(log::Level::Debug) { log::debug!("\n{:?}", req); }
        }
    }
    true
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
fn rule_out_static_resources<'a>(path: String) -> Response<'a> {
    let mut res: Response = Response::not_found();
    if path.contains('.') {
        match res.set_body_to_file(&path) {
            Ok(_) => {
                res.set_ok();
            },
            Err(e) => {
                res.set_not_found();
                if log::log_enabled!(log::Level::Error) {
                    log::error!("{}", e);
                }
            }
        }
    }
    res
}