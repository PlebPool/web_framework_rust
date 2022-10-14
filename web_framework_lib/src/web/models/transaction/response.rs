use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Error;
use std::ops::Add;
use std::str::FromStr;

use crate::web::util::enums::mime_types::MimeTypes;

const DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";

/// `Response` is a struct that contains a reference to a string, a `u16`, another reference to a
/// string, a `HashMap` of references to strings and strings, and a `Vec` of `u8`s.
///
/// The `'a` is a lifetime parameter. It's a way of saying that the struct contains references to data
/// that will live at least as long as the struct itself.
///
/// The `&'a str` is a reference to a string that will live at least as long as the struct itself.
///
/// The `HashMap
///
/// Properties:
///
/// * `protocol`: The protocol used for the response, e.g. HTTP/1.1
/// * `status`: The HTTP status code.
/// * `reason_phrase`: The reason phrase is a human-readable string that is usually
/// * `headers`: A HashMap of the headers in the response.
/// * `body`: The body of the response.
pub struct Response<'a> {
    protocol: &'a str,
    status: u16,
    reason_phrase: &'a str,
    headers: HashMap<&'a str, String>,
    body: Vec<u8>
}

impl Debug for Response<'_> {
    fn fmt<'a>(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let body_as_uft8: Cow<str>;
        body_as_uft8 = String::from_utf8_lossy(self.body().as_slice());
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
    /// `new` is a function that takes a `status` and a `reason_phrase` and returns a `Response` struct
    ///
    /// Arguments:
    ///
    /// * `status`: The HTTP status code.
    /// * `reason_phrase`: The reason phrase is a human-readable string that is usually used to explain
    /// the status code.
    ///
    /// Returns:
    ///
    /// A new instance of the Response struct.
    pub fn new(status: u16, reason_phrase: &'a str) -> Self {
        Self {
            protocol: DEFAULT_HTTP_VERSION,
            status,
            reason_phrase,
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    /// > This function creates a new `Response` struct with default values
    ///
    /// Returns:
    ///
    /// A new instance of the Response struct.
    pub fn new_empty() -> Self {
        Self {
            protocol: DEFAULT_HTTP_VERSION,
            status: 0,
            reason_phrase: "",
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    /// `ok()` returns a new `Response` with a status code of 200 and a status message of "OK"
    ///
    /// Returns:
    ///
    /// A new instance of the `Response` struct.
    pub fn ok() -> Self {
        Self::new(200, "OK")
    }

    /// `not_found()` returns a new `Status` with a code of `404` and a message of `Not Found`
    ///
    /// Returns:
    ///
    /// A new instance of the `HttpResponse` struct.
    pub fn not_found() -> Self {
        Self::new(404, "Not Found")
    }

    /// `bad_request` returns a `Response` with a status code of 400 and a body of `s`
    ///
    /// Arguments:
    ///
    /// * `s`: &str - The string to be sent as the body of the response.
    ///
    /// Returns:
    ///
    /// A new Response object with a status code of 400 and a body of the string passed in.
    pub fn bad_request(s: &str) -> Self {
        let mut res: Response = Self::new(400, "Bad Request");
        res.set_body(s.to_owned());
        res
    }

    /// > This function takes a path to a file in the `public` directory and sets the response body to
    /// the contents of that file, it also sets the "Content-Type" header based on file ext.
    ///
    /// Arguments:
    ///
    /// * `path_from_public`: The path to the file, relative to the public folder.
    ///
    /// Returns:
    ///
    /// A Result<(), Error>
    /// # Examples
    /// ```
    /// use web_framework_lib::web::server::data::models::transaction::response::Response;
    /// let mut res: Response = Response::new_empty();
    /// res.set_body_to_file("index.html").expect("");
    /// res.set_body_to_file("/index.html").expect("");
    pub fn set_body_to_file(&mut self, path_from_public: &str) -> Result<(), Error> {
        let mut path_prefix: String = "src/public".to_string();
        let mime_type: String = path_from_public.rsplit_once('.')
            .map(|(_parent_path, ext): (&str, &str)| {
                let ext: String = ext.replace(".download", "");
                let e: MimeTypes = MimeTypes::from_str(&ext)
                    .unwrap_or_else(|()| panic!("Invalid ext: {}", ext));
                e.to_string()
            }).expect("Failed to get mime type.");
        if !path_from_public.starts_with('/') { path_prefix = path_prefix.add("/"); }
        match fs::read(path_prefix.add(path_from_public)) {
            Ok(t) => {
                self.set_body_u8(t);
                self.add_header("Content-Type", mime_type);
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    /// `add_header` takes a mutable reference to a `Request` struct and two strings, one of which is
    /// borrowed, and inserts the key and value into the `headers` field of the `Request` struct
    ///
    /// Arguments:
    ///
    /// * `key`: &'a str
    /// * `val`: String - The value of the header.
    pub fn add_header(&mut self, key: &'a str, val: String) {
        self.headers.insert(key, val);
    }

    pub fn content_type(&mut self, mime: MimeTypes) {
        self.headers.insert("Content-Type", mime.to_string());
    }

    /// It sets the body of the response to the given string.
    ///
    /// Arguments:
    ///
    /// * `body`: The body of the request.
    pub fn set_body(&mut self, body: String) {
        self.set_body_u8(Vec::from(body.as_bytes()));
    }

    /// It sets the body of the response to a vector of u8.
    ///
    /// Arguments:
    ///
    /// * `body`: The body of the request.
    pub fn set_body_u8(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// It takes the response object and converts it into a vector of bytes
    ///
    /// Returns:
    ///
    /// A vector of bytes.
    pub fn get_as_u8_vec(&mut self) -> Vec<u8> {
        if self.status == 0 {
            panic!("Please mutate http status before getting as byte vector.")
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
        res_as_u8_vec.append(&mut self.body.clone()); // Cloning for debugging purposes.
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

    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_reason_phrase(&mut self, reason_phrase: &'a str) -> &mut Self {
        self.reason_phrase = reason_phrase;
        self
    }
}