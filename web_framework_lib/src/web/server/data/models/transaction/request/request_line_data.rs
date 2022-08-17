use std::str::SplitWhitespace;

#[derive(Debug)]
pub struct RequestLineData {
    method: String,
    pub(crate) path: String,
    protocol: String,
    pub(crate) path_query_bypassed: bool
}

#[allow(dead_code)]
impl <'a> RequestLineData {
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