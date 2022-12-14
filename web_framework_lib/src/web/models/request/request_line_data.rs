use std::collections::HashMap;
use std::str::{Split, SplitWhitespace};

use crate::web::models::request::request_line_data::request_queries::RequestQueries;
use crate::web::util::encoders::url_encoder;
use crate::web::util::parsers::request_parser::RequestParseError;

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

mod request_queries;

/// It's a struct that holds the method, path, and protocol of a request line.
///
/// The method is a string that holds the HTTP method of the request.
///
/// The path is a string that holds the path of the request.
///
/// The protocol is a string that holds the protocol of the request.
///
/// The path_query_bypassed is a boolean that holds whether or not the path has been bypassed.
///
/// The path_query_bypassed is a boolean that holds whether or not the path has been bypassed.
///
/// The path_query
///
/// Properties:
///
/// * `method`: The HTTP method used in the request.
/// * `path`: The path of the request.
/// * `protocol`: The protocol used in the request.
/// * `path_query_bypassed`: This is a boolean that indicates whether the path query has been bypassed.
#[derive(Debug)]
pub struct RequestLineData {
    method: String,
    path: String,
    protocol: String,
    request_queries: Option<RequestQueries>
}

#[allow(dead_code)]
impl RequestLineData {

    /// It takes a string, splits it by whitespace, and then assigns the first, second, and third
    /// elements to the method, path, and protocol fields of the Request struct
    ///
    /// Arguments:
    ///
    /// * `req_str_first_line`: The first line of the request string.
    ///
    /// Returns:
    ///
    /// A new instance of the Request struct.
    pub fn new(req_str_first_line: &str) -> Result<Self, RequestParseError> {
        let mut sws: SplitWhitespace = req_str_first_line.split_whitespace();
        let method: String =
            match sws.next() {
                Some(method) => {
                    Ok(method)
                },
                None => {
                    Err(RequestParseError::NoMethod)
                }
            }?.to_string();
        let full_path_string: String =
            match sws.next() {
                Some(path) => {
                    Ok(path)
                },
                None => {
                     Err(RequestParseError::NoPath)
                }
            }?.to_string();
        let path: String;
        let path_query_split_opt: Option<(&str, &str)> = full_path_string.split_once('?');
        let request_queries_opt: Option<RequestQueries> = match path_query_split_opt {
            Some((parent_path, queries_str)) => {
                path = parent_path.to_string();
                let mut map: HashMap<String, String> = HashMap::new();
                let query_iterator: Split<char> = queries_str.split('&');
                let _ = query_iterator.map(|s: &str| {
                    let key_val_pair: Option<(&str, &str)> = s.split_once('=');
                    if let Some((key, val)) = key_val_pair {
                        map.insert(key.to_string(), val.to_string());
                    }
                });
                Some(RequestQueries::new(map))
            },
            None => {
                path = full_path_string.to_string();
                None
            }
        };
        let protocol: String =
            match sws.next() {
                Some(protocol) => {
                    Ok(protocol)
                },
                None => {
                    Err(RequestParseError::NoProtocol)
                }
            }?.to_string();
        Ok(Self {
            method,
            path,
            protocol,
            request_queries: request_queries_opt
        })
    }

    pub fn get_path_cell_by_index_url_encoded(&self, index: usize) -> Option<String> {
        self.path.split('/')
            .filter(|s: &&str| !s.is_empty())
            .collect::<Vec<&str>>()
            .get(index)
            .map(|s: &&str| String::from(*s))
    }

    pub fn get_path_cell_by_index_url_decoded(&self, index: usize) -> Option<String> {
        self.get_path_cell_by_index_url_encoded(index)
            .map(|s: String| url_encoder::decode(&s))
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn request_queries(&self) -> &Option<RequestQueries> {
        &self.request_queries
    }
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }
    pub fn set_request_queries(&mut self, request_queries: Option<RequestQueries>) {
        self.request_queries = request_queries;
    }
}