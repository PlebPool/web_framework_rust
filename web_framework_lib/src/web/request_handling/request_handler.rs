use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Instant;
use di_ioc_lib::di::ioc_container::IocContainer;
use crate::web::model::request::Request;

pub fn handle(tcp_stream: std::io::Result<TcpStream>, container: Arc<IocContainer>) {
    let start_time: Instant = Instant::now();
    if tcp_stream.is_err() {
        unimplemented!();
    }
    let mut tcp_stream: TcpStream = tcp_stream.expect("TcpStream.unwrap() failed.");
    let mut input_buf: [u8; 1024] = [0; 1024]; // TODO: Unsure about buffer sizing.
    let _ = tcp_stream.read(&mut input_buf);
    let request: Request = Request::new_from_u8(&input_buf, Some(tcp_stream));

    let path: &str = request.request_line().path();
    println!("uwu")
}