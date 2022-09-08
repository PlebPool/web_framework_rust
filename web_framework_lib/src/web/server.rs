use di_ioc_lib::di::ioc_container::IocContainer;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::web::models::transaction::Transaction;
use crate::web::request_handling::chain_handler;
use crate::web::util::request_parser;

pub type HandlerFunction = fn(transaction: &mut Transaction);

pub fn start(port: &str, container: Arc<IocContainer>) {
    env_logger::init();
    let listener: TcpListener = TcpListener::bind("127.0.0.1:".to_owned() + port)
        .expect("BIND FAILED");

    for tcp_stream in listener.incoming() {

        let thread_builder: thread::Builder = thread::Builder::new()
            .name(String::from("REQUEST_HANDLER_THREAD"));

        let container: Arc<IocContainer> = Arc::clone(&container);

        thread_builder.spawn(move || {
            let transaction: Transaction = request_parser
            ::parse_request(
                tcp_stream.expect("Failed to unwrap tcp stream"),
                [0; 1024]
            );

            chain_handler::enter_chain(transaction, container);
        }).expect("Failed to spawn request handler thread.");
    }
}


