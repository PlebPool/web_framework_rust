use std::str::FromStr;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "POST" => { Ok(HttpMethod::POST) },
            "GET" => { Ok(HttpMethod::GET) },
            "PUT" => { Ok(HttpMethod::PUT) },
            "DELETE" => { Ok(HttpMethod::DELETE) },
            _ => { Err(()) }
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => { "GET".to_string() },
            HttpMethod::POST => { "POST".to_string() },
            HttpMethod::PUT => { "PUT".to_string() },
            HttpMethod::DELETE => { "DELETE".to_string() }
        }
    }
}