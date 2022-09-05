use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use di_ioc_lib::di::providable_trait::Providable;
use crate::web::server::HandlerFunction;

/// `RouteHandlerContainer` is a struct that contains a `HashMap` of `Regex` and `HandlerFunction`
/// `Regex` is a semantic struct that contains a `String` field.
///
/// Properties:
///
/// * `map`: This is a HashMap that will store the regular expression and the handler function.
pub struct RouteHandlerContainer {
    path_map: HashMap<String, HandlerFunction>,
}

impl Providable for RouteHandlerContainer { }

impl RouteHandlerContainer {
    pub fn new() -> Self {
        Self { path_map: HashMap::new() }
    }

    pub fn get(&self, path: &str) -> Option<&HandlerFunction> {
        self.path_map.get(path)
    }

    pub fn insert(&mut self, k: String, v: HandlerFunction) {
        self.path_map.insert(k, v);
    }
}

#[cfg(test)]
mod test {
    use crate::web::models::transaction::Transaction;
    use crate::web::request_handling::route_handler_container::RouteHandlerContainer;

    fn dummy(_t: &mut Transaction) { }

    #[test]
    fn test() {
        let mut rhc = RouteHandlerContainer::new();
        rhc.insert("/hey/test".to_string(), dummy);
        rhc.insert("/hey/{param}/test".to_string(), dummy);

    }
}
