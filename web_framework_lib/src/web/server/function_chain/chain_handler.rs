use crate::web::server::data::models::transaction::Transaction;


// TODO: Impl chain.
// TODO: Build chain.
// TODO: Execute chain.
// TODO: Resolve transaction.
// TODO: Log transaction to terminal.

mod handlers {
    pub mod handler_config;
    pub mod static_resource_handler;
}

pub fn enter_chain(mut transaction: Transaction) {
    let path = transaction.req().request_line_data().path.to_owned();
    let res = transaction.res_mut();
    if path == "/" {
        res.set_status(200);
        res.set_reason_phrase("OK");
        res.set_body("Hello".to_string());
        res.add_header("Content-Type", "text/plain".to_string());
    } else {
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