use std::io::Read;
use std::net::TcpStream;

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
    let buf: Vec<u8> = buf.iter()
        .filter(|byte|byte != &&0x0D && byte != &&0x0).map(|b|*b).collect();
    // Checking for delimiting double \n between headers and body.
    let mut previous_was_newline: bool = false;
    let mut index_for_split: usize = buf.len();
    for (index, val) in buf.iter().enumerate() {
        if val == &0x0A {
            if previous_was_newline {
                index_for_split = index;
                break;
            }
            previous_was_newline = true;
        } else {
            previous_was_newline = false;
        }
    }
    let (headers, body): (&[u8], &[u8]) = buf.split_at(index_for_split);
    // Removing trailing and prefixing newlines.
    let (headers, body): (&[u8], &[u8]) = (&headers[..headers.len()-1], &body[1..]);
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("\nHeaders: \n{:#?},\nBody: \n{:#?},\nEntire: \n{:#?}\n",
            String::from_utf8_lossy(headers),
            String::from_utf8_lossy(body),
            String::from_utf8_lossy(&buf)
        );
    }
    let req: Request = Request::new(
        headers,
        body,
        tcp_stream
    );
    let res: Response = Response::new_empty();
    Transaction::new(req, res)
}
