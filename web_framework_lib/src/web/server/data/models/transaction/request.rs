use std::collections::HashMap;
use std::net::TcpStream;
use crate::web::server::data::models::transaction::request::request_line_data::RequestLineData;

mod request_line_data;

#[derive(Debug)]
pub struct Request {
    request_line_data: RequestLineData,
    request_header_map: HashMap<String, String>,
    stream: TcpStream
}

#[allow(dead_code)]
impl <'a> Request {
    pub fn new(req_str: String, stream: TcpStream) -> Self {
        let mut req_split_new_line: Vec<&str> = req_str.split('\n').collect();
        req_split_new_line.reverse();
        Self {
            request_line_data:
            RequestLineData
            ::new(req_split_new_line.pop().expect("No first line")),
            request_header_map: Self::req_str_to_map(req_split_new_line.to_owned()),
            stream
        }
    }

    fn req_str_to_map(req_str: Vec<&str>) -> HashMap<String, String> {
        let mut req_header_map: HashMap<String, String> = HashMap::new();
        for x in req_str.into_iter() {
            match x.split_once(':') {
                Some((hdr_key, hdr_val)) => {
                    req_header_map.insert(hdr_key.trim().to_string(), hdr_val.trim().to_string());
                },
                None => {}
            }
        }
        req_header_map
    }

    // TODO: TMP.
    pub fn cut_query(&mut self) {
        let query_split_off: Option<(&str, &str)> =
            self.request_line_data.path.split_once('?');
        if let Some((path, _query_str)) = query_split_off {
            self.request_line_data.set_path(String::from(path));
            self.request_line_data.set_path_query_bypassed(true);
        }
    }

    pub fn request_line_data(&self) -> &RequestLineData {
        &self.request_line_data
    }
    pub fn request_header_map(&self) -> &HashMap<String, String> {
        &self.request_header_map
    }
    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn set_request_line_data(&mut self, request_line_data: RequestLineData) {
        self.request_line_data = request_line_data;
    }
    pub fn set_request_header_map(&mut self, request_header_map: HashMap<String, String>) {
        self.request_header_map = request_header_map;
    }
    pub fn set_stream(&mut self, stream: TcpStream) {
        self.stream = stream;
    }
}