use std::collections::HashMap;
use std::string::FromUtf8Error;

/// `HttpHeaders` is a struct that contains a `HashMap` of `String`s.
///
/// The `HashMap` is a collection type that maps keys to values. In this case, the keys are `String`s
/// and the values are also `String`s.
///
/// The `HashMap` is a generic type, meaning that it can be used to map any type to any other type.
///
/// The `HashMap` is a standard library type, meaning that it's part of the Rust language.
///
/// The `HashMap` is a mutable
///
/// Properties:
///
/// * `headers`: This is a HashMap that will hold the headers.
#[derive(Debug)]
pub struct HttpHeaders {
    headers: HashMap<String, String>
}

impl HttpHeaders {
    pub fn new(headers: HashMap<String, String>) -> Self {
        Self { headers }
    }

    pub fn new_from_u8(data: Vec<&[u8]>) -> Self {
        // TODO: Add constructor for non newline split arguments.
        // let newline_split: Vec<&[u8]> = data.split(|b: &u8| *b == 10).collect::<Vec<&[u8]>>();
        let headers: HashMap<String, String> = data
            .into_iter().map(|b: &[u8]| {
                // Getting position of first colon.
                let pos: Option<usize> = b.iter().position(|b: &u8| *b == 58);
                if pos.is_none() {
                    // TODO: Handle.
                    panic!("{} || {:?}", String::from_utf8_lossy(b), ":".as_bytes());
                } else {
                    // Splitting at first colon.
                    let (key, val): (&[u8], &[u8]) = b.split_at(pos.unwrap());
                    // Removing recidural colon in value.
                    let (key, val): (&[u8], &[u8]) = (&key[..key.len()], &val[1..val.len()]);
                    let key: Result<String, FromUtf8Error> = String::from_utf8(key.to_vec());
                    let val: Result<String, FromUtf8Error> = String::from_utf8(val.to_vec());
                    if let Err(e) = key { panic!("{}", e); } // TODO: Handle
                    if let Err(e) = val { panic!("{}", e); } // TODO: Handle
                    let key: String = key.unwrap();
                    let val: String = val.unwrap();
                    (key.trim().to_string(), val.trim().to_string())
                }
            }).collect::<HashMap<String, String>>();
        Self::new(headers)
    }
}

#[cfg(test)]
mod test {
    use crate::web::model::http_headers::HttpHeaders;

    const TEST_STR: &str = r#"Host: localhost:8000
    User-Agent: Mozilla/5.0 (Macintosh; ... ) ... Firefox/51.0
    Accept: text/html,application/xhtml+xml,...,*/*;q=0.8
    Accept-Language: en-US,en,q=0.5
    Accept-Encoding: gzip, deflate
    Connection: keep-alive
    Upgrade-Insecure-Requests: 1
    Content-Type: multipart/form-data; boundary=-12656974
    Content-Length: 35"#;

    #[test]
    fn constructor_test() {
        dbg!(HttpHeaders::new_from_u8(TEST_STR.as_bytes().split(|b: &u8| *b == 10).collect()));
    }
}