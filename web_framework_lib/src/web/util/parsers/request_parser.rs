
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::web::models::request::Request;
use crate::web::models::response::Response;

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
pub enum RequestParseError {
    NoMethod,
    NoPath,
    NoProtocol
}

impl ToString for RequestParseError {
    fn to_string(&self) -> String {
        match self {
            RequestParseError::NoProtocol => {
                "No protocol"
            },
            RequestParseError::NoPath => {
                "No path"
            },
            RequestParseError::NoMethod => {
                "No method"
            }
        }.to_string()
    }
}

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
pub fn parse_request<'a>(mut tcp_stream: TcpStream, mut buf: [u8; 1024]) -> Result<Request, RequestParseError> {
    tcp_stream.read(&mut buf).expect("TcpStream read failed");
    let buf: Vec<u8> = buf.into_iter()
        .filter(|byte: &u8|*byte != 13 && *byte != 0).collect::<Vec<u8>>();
    // Checking for delimiting double \n between headers and body.
    let mut previous_was_newline: bool = false;
    let mut index_for_split: usize = buf.len();
    for (index, val) in buf.iter().enumerate() {
        if *val == 10 {
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
        log::debug!("\nHeaders: \n{:#?},\nBody: \n{:#?},\n",
            String::from_utf8_lossy(headers),
            String::from_utf8_lossy(body),
        );
    }
    let tcp_clone: std::io::Result<TcpStream> = tcp_stream.try_clone();
    Request::new(headers, body, tcp_stream)
}
