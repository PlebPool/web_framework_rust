use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{LockResult, Mutex, MutexGuard};

use crate::web::models::request::request_headers::RequestHeaders;
use crate::web::models::request::request_line_data::RequestLineData;
use crate::web::models::response::Response;
use crate::web::request_handling::request_handler::HandleError;
use crate::web::util::parsers::json_parser::{JsonObject, JsonParseError, parse_into_json_object};
use crate::web::util::parsers::request_parser::RequestParseError;

mod request_line_data;
mod request_headers;

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

/// `Request` is a struct that contains a `RequestLineData` struct, a `HashMap` of `String`s, and a
/// `TcpStream`.
///
/// Properties:
///
/// * `request_line_data`: This is a struct that contains the request line data.
/// * `request_header_map`: This is a HashMap that will contain the request headers.
/// * `stream`: The stream of data that the request is coming in on.
#[derive(Debug)]
pub struct Request {
    request_line_data: RequestLineData,
    request_headers: Option<RequestHeaders>,
    body: Vec<u8>,
    stream: TcpStream,
    resolved: Mutex<bool>
}

#[allow(dead_code)]
impl Request {
    /// This function takes a string and a stream, and returns a Request struct
    ///
    /// Arguments:
    ///
    /// * `req_str`: The request string that was sent to the server.
    /// * `stream`: The stream that the request was received on.
    ///
    /// Returns:
    ///
    /// A new instance of the Request struct.
    pub fn new(req_line_data_and_headers: &[u8], body: &[u8], stream: TcpStream) -> Result<Self, RequestParseError> {
        let lossy_utf8: Cow<str> = String::from_utf8_lossy(req_line_data_and_headers);
        let mut req_split_new_line: Vec<&str> = lossy_utf8.lines().collect();
        req_split_new_line.reverse();
        let request_line_data: RequestLineData = RequestLineData::new(req_split_new_line.pop().expect("No first line"))?;
        let request_headers: Option<RequestHeaders> = RequestHeaders::new(Self::req_str_to_header_map(req_split_new_line.to_owned()));
        Ok(Self {
            request_line_data,
            request_headers,
            body: Vec::from(body),
            stream,
            resolved: Mutex::new(false)
        })
    }

     pub fn get_body_as_json<'a>(&self) -> Result<JsonObject, JsonParseError> {
         let as_json: Result<JsonObject, JsonParseError> =
             parse_into_json_object(self.body.as_slice());
         match as_json {
             Ok(json_object) => { Ok(json_object) }
             Err(e) => {
                 Err(e)
             }
         }
     }

    pub fn resolve(&self, mut res: Response) -> Result<(), HandleError> {
        let mutex_lock: LockResult<MutexGuard<bool>> = self.resolved.lock();
        if let Ok(mut t) = mutex_lock {
            return if *t {
                Err(HandleError::AlreadyResolved)
            } else if res.status() == 0 {
                Err(HandleError::HttpStatusNotSet)
            } else {
                let write_result: std::io::Result<usize> = self.stream()
                    .write(res.get_as_u8_vec()
                        .as_slice());
                if write_result.is_err() {
                    return Err(HandleError::ResolveFailed)
                }
                *t = true;
                Ok(())
            }
        } else {
            Err(HandleError::UnobtainedMutex)
        }
    }

    /// It takes a vector of strings, and returns a hashmap of strings
    ///
    /// Arguments:
    ///
    /// * `req_str`: A vector of strings, each string is a header line.
    ///
    /// Returns:
    ///
    /// A HashMap<String, String>
    fn req_str_to_header_map(req_str: Vec<&str>) -> HashMap<String, String> {
        let mut req_header_map: HashMap<String, String> = HashMap::new();
        for x in req_str.into_iter() {
            match x.split_once(':') {
                Some((hdr_key, hdr_val)) => {
                    req_header_map.insert(hdr_key.trim().to_string(),
                                          hdr_val.trim().to_string());
                },
                None => {}
            }
        }
        req_header_map
    }

    pub fn request_line_data(&self) -> &RequestLineData {
        &self.request_line_data
    }
    pub fn request_headers(&self) -> &Option<RequestHeaders> {
        &self.request_headers
    }
    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }
    pub fn resolved(&self) -> &Mutex<bool> {
        &self.resolved
    }

    pub fn set_request_line_data(&mut self, request_line_data: RequestLineData) {
        self.request_line_data = request_line_data;
    }
    pub fn set_request_headers(&mut self, request_headers: Option<RequestHeaders>) {
        self.request_headers = request_headers;
    }
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
    pub fn set_stream(&mut self, stream: TcpStream) {
        self.stream = stream;
    }
    pub fn set_resolved(&mut self, resolved: Mutex<bool>) {
        self.resolved = resolved;
    }
}