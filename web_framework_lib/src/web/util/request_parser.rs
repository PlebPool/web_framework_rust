use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;
use crate::web::models::transaction::request::Request;
use crate::web::models::transaction::response::Response;
use crate::web::models::transaction::Transaction;

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
    let req_as_str: String = from_utf8(&buf).expect("Failed to parse buffer to utf8").to_owned();
    let req: Request = Request::new(
        req_as_str,
        tcp_stream
    );
    let res: Response = Response::new_empty();
    Transaction::new(req, res)
}
