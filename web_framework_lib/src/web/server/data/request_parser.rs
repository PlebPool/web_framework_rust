use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;
use crate::web::server::data::models::transaction::request::Request;
use crate::web::server::data::models::transaction::response::Response;
use crate::web::server::data::models::transaction::Transaction;

/// It reads a buffer from a TcpStream, parses it into a Request, creates a new empty Response, and
/// returns a Transaction
///
/// Arguments:
///
/// * `tcp_stream`: The TcpStream that we're reading from.
/// * `buf`: [u8; 1024]
///
/// Returns:
///
/// A Transaction struct
pub fn parse_request<'a>(mut tcp_stream: TcpStream, mut buf: [u8; 1024]) -> Transaction<'a> {
    tcp_stream.read(&mut buf)
        .expect("TcpStream read failed");
    let i = from_utf8(&buf).expect("Failed to parse buffer to utf8").to_owned();
    let req: Request = Request::new(
        i,
        tcp_stream
    );
    let res: Response = Response::new_empty();
    Transaction::new(req, res)
}
