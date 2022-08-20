use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Error;
use std::ops::Add;

const DEFAULT_HTTP_VERSION: &str = "HTTP/2";

pub struct Response<'a> {
    protocol: &'a str,
    status: u16,
    reason_phrase: &'a str,
    headers: HashMap<&'a str, String>,
    body: Vec<u8>
}

impl Debug for Response<'_> {
    fn fmt<'a>(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let body_as_uft8;

        body_as_uft8 = String::from_utf8_lossy(&self.body);

        f.debug_struct("Response")
            .field("protocol", &self.protocol)
            .field("status", &self.status)
            .field("reason_phrase", &self.reason_phrase)
            .field("headers", &self.headers)
            .field("body", &body_as_uft8)
            .finish()
    }
}
#[allow(dead_code)]
impl <'a> Response<'a> {
    pub fn new(status: u16, reason_phrase: &'a str) -> Self {
        Self {
            protocol: DEFAULT_HTTP_VERSION,
            status,
            reason_phrase,
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    pub fn new_empty() -> Self {
        Self {
            protocol: DEFAULT_HTTP_VERSION,
            status: 0,
            reason_phrase: "",
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    pub fn ok() -> Self {
        Self::new(200, "OK")
    }

    pub fn not_found() -> Self {
        Self::new(404, "Not Found")
    }

    pub fn bad_request(s: &str) -> Self {
        let mut res: Response = Self::new(400, "Bad Request");
        res.set_body(s.to_owned());
        res
    }

    pub fn set_body_to_file(&mut self, path: &str) -> Result<(), Error> {
        match fs::read(path) {
            Ok(t) => {
                self.set_body_u8(t);
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn add_header(&mut self, key: &'a str, val: String) {
        self.headers.insert(key, val);
    }

    pub fn set_body(&mut self, body: String) {
        self.set_body_u8(Vec::from(body.as_bytes()));
    }

    pub fn set_body_u8(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn get_as_u8_vec(&mut self) -> Vec<u8> {
        if self.status == 0 {
            panic!("Response status was never mutated but you're attempting to writing a body for it.")
        }
        let mut header_map_to_str: String = String::new();
        for (k, v) in &self.headers {
            header_map_to_str = header_map_to_str
                .add(k).add(": ").add(v.as_str()).add("\r\n")
        };
        let mut res_as_u8_vec: Vec<u8> =
            Vec::from(format!("{proto} {status} {reason}\r\n{headers}\r\n",
                              proto=self.protocol,
                              status=self.status,
                              reason=self.reason_phrase,
                              headers=header_map_to_str).as_bytes());
        res_as_u8_vec.append(&mut self.body);
        res_as_u8_vec
    }

    pub fn protocol(&self) -> &str {
        self.protocol
    }
    pub fn status(&self) -> u16 {
        self.status
    }
    pub fn reason_phrase(&self) -> &str {
        self.reason_phrase
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn headers(&self) -> &HashMap<&'a str, String> {
        &self.headers
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_reason_phrase(&mut self, reason_phrase: &'a str) {
        self.reason_phrase = reason_phrase;
    }
}