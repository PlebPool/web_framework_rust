use di_ioc_lib::di::ioc_container::IocContainer;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::web::models::transaction::Transaction;
use crate::web::request_handling::request_handler;
use crate::web::util::request_parser;

pub type HandlerFunction = fn(transaction: &mut Transaction);

pub fn start(port: &str, container: Arc<IocContainer>) {
    let _ = env_logger::try_init();

    let listener: TcpListener = TcpListener::bind("127.0.0.1:".to_owned() + port)
        .expect("BIND FAILED");

    for tcp_stream in listener.incoming() {

        let thread_builder: thread::Builder = thread::Builder::new()
            .name(String::from("REQUEST_HANDLER_THREAD"));

        let container_reference_clone: Arc<IocContainer> = Arc::clone(&container);

        thread_builder.spawn(move || {

            // We pass the TcpStream and a buffer to the parser. It returns an initialized transaction.
            let transaction: Transaction = request_parser::parse_request(
                tcp_stream.expect("Failed to unwrap tcp stream"),
                [0; 1024]
            );

            if log::log_enabled!(log::Level::Info) {
                log::info!("Request Received from {}", transaction.req().stream().peer_addr().unwrap());
            }

            if log::log_enabled!(log::Level::Debug) {
                log::debug!("{:#?}", transaction.req());
            }

            // Pass container reference and parsed transaction.
            // TODO: Maybe extract RouteHandlerContainer here already, or earlier.
            request_handler::enter_chain(transaction, container_reference_clone);
        }).expect("Failed to spawn request handler thread.");
    }
}


