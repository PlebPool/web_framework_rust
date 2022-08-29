use std::collections::HashMap;
use std::str::{Split, SplitWhitespace};
use crate::web::server::data::models::transaction::request::request_line_data::request_queries::RequestQueries;

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
    pub(crate) path: String,
    protocol: String,
    pub(crate) path_query_bypassed: bool,
    request_queries: Option<RequestQueries>,
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
    pub fn new(req_str_first_line: &str) -> Self {
        let mut sws: SplitWhitespace = req_str_first_line.split_whitespace();
        let method =
            match sws.next() { Some(t) => { t }, None => { "[NO_METHOD]" } }.to_string();
        let full_path_string =
            match sws.next() { Some(t) => { t }, None => { "[NO_PATH]" } }.to_string();
        let path: String;
        let path_query_split_opt: Option<(&str, &str)> = full_path_string.split_once('?');
        let request_queries_opt: Option<RequestQueries> = match path_query_split_opt {
            Some((parent_path, queries_str)) => {
                path = parent_path.to_string();
                let mut map: HashMap<String, String> = HashMap::new();
                let query_iterator: Split<char> = queries_str.split('&');
                for key_val_pair in query_iterator {
                    let key_val_pair: Option<(&str, &str)> = key_val_pair.split_once('=');
                    if let Some((key, val)) = key_val_pair {
                        map.insert(key.to_string(), val.to_string());
                    }
                }
                Some(RequestQueries::new(map))
            },
            None => {
                path = full_path_string.to_string();
                None
            }
        };
        let protocol =
            match sws.next() { Some(t) => { t }, None => { "[NO_PROTOCOL]" } }.to_string();
        Self {
            method,
            path,
            protocol,
            path_query_bypassed: false,
            request_queries: request_queries_opt
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn path_query_bypassed(&self) -> bool {
        self.path_query_bypassed
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
    pub fn set_path_query_bypassed(&mut self, path_query_bypassed: bool) {
        self.path_query_bypassed = path_query_bypassed;
    }
    pub fn set_request_queries(&mut self, request_queries: Option<RequestQueries>) {
        self.request_queries = request_queries;
    }
}