use std::io::Write;
use di_ioc_lib::di::ioc_container::IocContainer;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::web::models::request::Request;
use crate::web::models::response::Response;

use crate::web::request_handling::request_handler;
use crate::web::util::parsers::request_parser;

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

const BANNER: &str = "
 ██████╗        █████╗       ███╗   ███╗      ███╗   ███╗       █████╗
██╔════╝       ██╔══██╗      ████╗ ████║      ████╗ ████║      ██╔══██╗
██║  ███╗█████╗███████║█████╗██╔████╔██║█████╗██╔████╔██║█████╗███████║
██║   ██║╚════╝██╔══██║╚════╝██║╚██╔╝██║╚════╝██║╚██╔╝██║╚════╝██╔══██║
╚██████╔╝      ██║  ██║      ██║ ╚═╝ ██║      ██║ ╚═╝ ██║      ██║  ██║
 ╚═════╝       ╚═╝  ╚═╝      ╚═╝     ╚═╝      ╚═╝     ╚═╝      ╚═╝  ╚═╝
███████████████████████████████████████████████████████████████████████

";

pub type HandlerFunction = fn(req: &Request) -> Response;

pub fn start(port: &str, container: Arc<IocContainer>) {
    let _ = env_logger::try_init();

    let listener: TcpListener = TcpListener::bind("127.0.0.1:".to_owned() + port)
        .expect("BIND FAILED");

    for banner_char in BANNER.chars() {
        print!("{}", banner_char);
        thread::sleep(Duration::from_millis(1));
        std::io::stdout().flush().unwrap();
    }

    for tcp_stream in listener.incoming() {
        let container_reference_clone: Arc<IocContainer> = Arc::clone(&container);

        thread::spawn(move || {
            // Pass container reference and parsed transaction.
            request_handler::handle(tcp_stream, container_reference_clone);
        });
    }
}


