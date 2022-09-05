use std::collections::HashMap;

/// `RequestQueries` is a struct that contains a `HashMap` of `String`s.
///
/// The `HashMap` is a data structure that maps keys to values. In this case, the keys are `String`s and
/// the values are also `String`s.
///
/// The `HashMap` is a generic type, which means that it can be used to map any type of key to any type
/// of value.
///
/// The `HashMap` is a standard library type, which means that it's part of the Rust language.
///
/// The `HashMap
///
/// Properties:
///
/// * `queries`: A HashMap that contains the query parameters.
/// `RequestQueries` is a struct that contains a `HashMap` of `String`s.
///
/// The `HashMap` is a data structure that maps keys to values. In this case, the keys are `String`s and
/// the values are also `String`s.
///
/// The `HashMap` is a generic type, which means that it can be used to map any type of key to any type
/// of value.
///
/// The `HashMap` is a standard library type, which means that it's part of the Rust language.
///
/// The `HashMap
///
/// Properties:
///
/// * `queries`: A HashMap that contains the query parameters.
#[derive(Debug)]
pub struct RequestQueries {
    queries: HashMap<String, String>
}

impl <'a> RequestQueries {
    pub fn new_empty() -> Self {
        Self { queries: HashMap::new() }
    }

    pub fn new(queries: HashMap<String, String>) -> Self {
        Self { queries }
    }

    /// This function takes a mutable reference to a `Request` struct, a string slice for the key, and a
    /// string slice for the value. It then inserts the key and value into the `queries` hashmap
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the query parameter.
    /// * `val`: &str - The value of the query parameter.
    pub fn add_query(&mut self, key: &str, val: &str) {
        self.queries.insert(key.to_string(), val.to_string());
    }

    /// It removes the query and returns the value.
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the query to remove.
    ///
    /// Returns:
    ///
    /// A string
    pub fn remove_query_and_get_val(&mut self, key: &str) -> Option<String> {
        self.queries.remove(key)
    }

    /// `remove_query` removes a query from the `queries` map
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the query to remove.
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub fn remove_query(&mut self, key: &str) -> bool {
        self.queries.remove(key).is_some()
    }
}