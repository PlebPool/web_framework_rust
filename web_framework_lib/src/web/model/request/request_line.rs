/// `RequestEntity` is a struct that has three fields, `method`, `path`, and `protocol`, each of which
/// is a `String`.
///
/// Properties:
///
/// * `method`: The HTTP method used to make the request.
/// * `path`: The path of the request.
/// * `protocol`: The protocol used to make the request.
#[derive(Debug)]
pub struct RequestLine {
    method: String,
    path: String,
    protocol: String
}

impl RequestLine {
    pub fn new(data: &[u8]) -> Self {
        let space_split: Vec<&[u8]> = data.split(|b: &u8| *b == 32).collect::<Vec<&[u8]>>();
        if space_split.len() != 3 { unimplemented!() }
        let method: String = String::from_utf8(space_split[0].to_vec())
            .expect("Method to string parse failed.");
        let path: String = String::from_utf8(space_split[1].to_vec())
            .expect("Path to string parse failed.");
        let protocol: String = String::from_utf8(space_split[2].to_vec())
            .expect("Protocol to string parse failed.");
        Self { method, path, protocol }
        // TODO: I WAS HERE 2!!!
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod test {
    use crate::web::model::request::request_line::RequestLine;

    const TEST_STR: &str = r#"GET /test HTTP/1.1"#;
    #[test]
    fn constructor_test() {
        let result: RequestLine = RequestLine::new(TEST_STR.as_bytes());
        dbg!(&result);
        assert_eq!(result.method, String::from("GET"));
        assert_eq!(result.path, String::from("/test"));
        assert_eq!(result.protocol, String::from("HTTP/1.1"));
    }
}