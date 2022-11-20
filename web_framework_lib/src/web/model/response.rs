use crate::web::model::http_headers::HttpHeaders;

/// `Response` is a struct that contains a reference to a string, a `u16`, another reference to a
/// string, an `HttpHeaders` struct, and an optional `Vec<u8>`.
/// 
/// The first two fields are the protocol and status code, respectively. The third field is the reason
/// phrase, which is a human-readable description of the status code. The fourth field is the headers,
/// which is a struct that contains a `HashMap` of header names to header values. The last field is the
/// body, which is an optional `Vec<u8>`
/// 
/// Properties:
/// 
/// * `response_protocol`: The protocol used in the response.
/// * `response_status`: The HTTP status code of the response.
/// * `response_reason_phrase`: The reason phrase of the response.
/// * `response_headers`: This is a HashMap that contains the response headers.
/// * `body`: The body of the response.
pub struct Response<'a> {
    response_protocol: &'a str,
    response_status: u16,
    response_reason_phrase: &'a str,
    response_headers: HttpHeaders,
    body: Option<Vec<u8>>
}