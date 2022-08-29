use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use di_ioc_lib::di::container::Container;
use crate::web::server::data::models::transaction::Transaction;
use crate::web::server::data::request_parser;
use crate::web::server::function_chain::chain_handler;

pub mod data {
    pub mod models {
        pub mod transaction;
    }
    pub mod enums {
        pub mod http_method_enum;
    }
    pub mod request_parser;
}


pub mod function_chain {
    pub mod route_handler_container;
    pub mod chain_handler;
}

pub type HandlerFunction = fn(transaction: &mut Transaction);

pub fn start(port: &str, container: Arc<Container>) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:".to_owned() + port)
        .expect("BIND FAILED");

    for tcp_stream in listener.incoming() {

        let thread_builder: thread::Builder = thread::Builder::new()
            .name(String::from("REQUEST_HANDLER_THREAD"));

        let container: Arc<Container> = Arc::clone(&container);

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


