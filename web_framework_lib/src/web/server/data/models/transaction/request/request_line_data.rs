use std::str::SplitWhitespace;

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
    pub(crate) path_query_bypassed: bool
}

#[allow(dead_code)]
impl <'a> RequestLineData {
    /// It takes a string, splits it on whitespace, and then assigns the first, second, and third
    /// elements to the method, path, and protocol fields of the Request struct
    ///
    /// Arguments:
    ///
    /// * `req_str_first_line`: The first line of the request.
    ///
    /// Returns:
    ///
    /// A new instance of the Request struct.
    pub fn new(req_str_first_line: &'a str) -> Self {
        let mut sws: SplitWhitespace = req_str_first_line.split_whitespace();

        let method =
            match sws.next() { Some(t) => { t }, None => { "[NO_METHOD]" } }.to_string();

        let path =
            match sws.next() { Some(t) => { t }, None => { "[NO_PATH]" } }.to_string();

        let protocol =
            match sws.next() { Some(t) => { t }, None => { "[NO_PROTOCOL]" } }.to_string();

        Self { method, path, protocol, path_query_bypassed: false }
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
}