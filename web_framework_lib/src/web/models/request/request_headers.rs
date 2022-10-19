use std::collections::HashMap;

//  █     █░▓█████   ▄████  ▄▄▄▄    ██▓     ▄▄▄      ▓█████▄
// ▓█░ █ ░█░▓█   ▀  ██▒ ▀█▒▓█████▄ ▓██▒    ▒████▄    ▒██▀ ██▌
// ▒█░ █ ░█ ▒███   ▒██░▄▄▄░▒██▒ ▄██▒██░    ▒██  ▀█▄  ░██   █▌
// ░█░ █ ░█ ▒▓█  ▄ ░▓█  ██▓▒██░█▀  ▒██░    ░██▄▄▄▄██ ░▓█▄   ▌
// ░░██▒██▓ ░▒████▒░▒▓███▀▒░▓█  ▀█▓░██████▒ ▓█   ▓██▒░▒████▓
// ░ ▓░▒ ▒  ░░ ▒░ ░ ░▒   ▒ ░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░ ▒▒▓  ▒
//   ▒ ░ ░   ░ ░  ░  ░   ░ ▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░ ░ ▒  ▒
//   ░   ░     ░   ░ ░   ░  ░    ░   ░ ░     ░   ▒    ░ ░  ░
//     ░       ░  ░      ░  ░          ░  ░      ░  ░   ░
//                               ░                    ░

/// `RequestHeaders` is a struct that contains a `HashMap` of `String`s.
///
/// The `HashMap` is a data structure that maps keys to values. In this case, the keys are `String`s and
/// the values are also `String`s.
///
/// The `HashMap` is a generic type, meaning that it can be used to map any type of key to any type of
/// value.
///
/// The `HashMap` is a standard library type, meaning that it's part of the Rust language.
///
/// The `HashMap` is
///
/// Properties:
///
/// * `headers`: A HashMap of the headers.
#[derive(Debug)]
pub struct RequestHeaders {
    headers: HashMap<String, String>
}

impl RequestHeaders {
    /// `new_empty` creates a new `Headers` instance with an empty `HashMap`
    ///
    /// Returns:
    ///
    /// A new instance of the `Headers` struct.
    pub fn new_empty() -> Self {
        Self { headers: HashMap::new() }
    }

    /// `new` is a function that takes a `HashMap<String, String>` and returns a `Self` (which is a
    /// `Header`)
    ///
    /// Arguments:
    ///
    /// * `headers`: A HashMap of the headers to be sent with the request.
    ///
    /// Returns:
    ///
    /// A new instance of the struct `Headers`
    pub fn new(headers: HashMap<String, String>) -> Self {
        Self { headers }
    }

    /// This function takes a mutable reference to a `Request` struct, and two strings, and inserts the
    /// two strings into the `headers` field of the `Request` struct
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the header.
    /// * `val`: The value of the header.
    pub fn add_header(&mut self, key: &str, val: &str) {
        self.headers.insert(key.to_string(), val.to_string());
    }

    /// It removes the header and returns the value of the header.
    ///
    /// Arguments:
    ///
    /// * `key`: &str - The key of the header to remove
    ///
    /// Returns:
    ///
    /// A string
    pub fn remove_header_and_get_val(&mut self, key: &str) -> Option<String> {
        self.headers.remove(key)
    }

    /// It removes the header from the headers map.
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the header to remove.
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub fn remove_header(&mut self, key: &str) -> bool {
        self.headers.remove(key).is_some()
    }
}