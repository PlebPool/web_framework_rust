
use std::net::TcpStream;
use std::sync::Mutex;
use crate::web::model::http_headers::HttpHeaders;
use crate::web::model::request::request_line::RequestLine;

pub mod request_line;

/// `Request` is a struct that contains a `RequestEntity`, a `Future` that resolves to an
/// `Option<HttpHeaders>`, a `Future` that resolves to a `Vec<u8>`, a `TcpStream`, and a `Mutex<bool>`.
///
/// The `RequestEntity` is a struct that contains a `String` and a `Vec<u8>`. The `String` is the
/// request URI, and the `Vec<u8>` is the request body.
///
/// The `Future` that resolves to an
///
/// Properties:
///
/// * `request_entity`: This is the RequestEntity struct that we defined earlier.
/// * `request_headers`: This is a future that will resolve to the HTTP headers of the request.
/// * `request_body`: This is a future that will resolve to the body of the request.
/// * `tcp_stream`: The TCP stream that the request is being sent over.
/// * `resolved`: This is a boolean that indicates whether the request has been resolved or not.
#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    request_headers: HttpHeaders,
    request_body: Option<Vec<u8>>,
    tcp_stream: Option<TcpStream>,
    resolved: Mutex<bool>
}

impl Request {
    pub fn new(request_line: RequestLine,
               request_headers: HttpHeaders,
               request_body: Option<Vec<u8>>,
               tcp_stream: Option<TcpStream>,
               resolved: Mutex<bool>) -> Self {
        Self {
            request_line,
            request_headers,
            request_body,
            tcp_stream,
            resolved
        }
    }

    // Takes a buffer CONTAINING the request, and the associated TcpStream.
    pub fn new_from_u8(buf: &[u8], tcp_stream: Option<TcpStream>) -> Self {
        let buf: Vec<u8> = Self::filter_bytes(buf);
        // pre_body = all bytes before the body, body = the bytes containing the body.
        let (pre_body, body): (&[u8], Option<&[u8]>) = Self::split_at_header_to_body_delim(&buf);
        let mut pre_body_newline_split: Vec<&[u8]> = pre_body
            .split(|b: &u8| *b == 10)
            .collect::<Vec<&[u8]>>();
        // We need to reverse it so that the pop effects the first element.
        pre_body_newline_split.reverse();
        let request_line: RequestLine = RequestLine::new(
            pre_body_newline_split.pop().expect("No first line.")
        );
        let request_headers: HttpHeaders = HttpHeaders::new_from_u8(
            pre_body_newline_split
        );
        let request_body: Option<Vec<u8>> = body.map(|b: &[u8]| b.to_vec());

        Self::new(
            request_line,
            request_headers,
            request_body,
            tcp_stream,
            Mutex::new(false)
        )
    }

    fn filter_bytes(input_buf: &[u8]) -> Vec<u8> {
        // 13 = CR-LF (Carriage Return (and) Line Feed), 0 = null.
        input_buf
            .into_iter()
            .map(|b: &u8| *b)
            .filter(|b: &u8| *b != 0 || *b != 13)
            .collect::<Vec<u8>>()
    }

    fn split_at_header_to_body_delim<'a>(buf: &'a [u8]) -> (&'a [u8], Option<&'a [u8]>) {
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
        let (headers, body_or_all): (&[u8], &[u8]) = buf.split_at(index_for_split);
        // If there is no body, all bytes will end up in the "body_or_all" variable.
        if headers.is_empty() {
            return (body_or_all, None);
        } else {
            // Removing trailing and prefixing newlines.
            (&headers[..headers.len()-1], Some(&body_or_all[1..]))
        }
    }

    pub fn request_line(&self) -> &RequestLine {
        &self.request_line
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::web::model::request::Request;

    const TEST_STR: &str = r#"
        GET /test HTTP/1.1
        Host: localhost:8000
        User-Agent: Mozilla/5.0 (Macintosh; ... ) ... Firefox/51.0
        Accept: text/html,application/xhtml+xml,...,*/*;q=0.8
        Accept-Language: en-US,en,q=0.5
        Accept-Encoding: gzip, deflate
        Connection: keep-alive
        Upgrade-Insecure-Requests: 1
        Content-Type: multipart/form-data; boundary=-12656974
        Content-Length: 35

        bookId=12345&author=Tan+Ah+Teck
    "#;

    #[test]
    fn constructor_test() {
        let start: Instant = Instant::now();
        dbg!(Request::new_from_u8(
            TEST_STR.trim().as_bytes(),
            None
        ));
        let duration: f64 = (Instant::now()
            .duration_since(start)
            .as_nanos() as f64)/(1_000_000 as f64);
        println!("Took: {}ms",
            duration
        );
    }
}

