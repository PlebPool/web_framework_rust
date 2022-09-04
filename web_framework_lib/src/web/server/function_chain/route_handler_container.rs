use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use di_ioc_lib::di::providable_trait::Providable;
use crate::web::server::HandlerFunction;

#[derive(Debug, PartialEq, Eq, Hash)]
struct RoutePath {
    regex_path: String,
    param_names_in_order: Vec<String>
}

impl RoutePath {
    pub fn new(regex_path: String, param_names_in_order: Vec<String>) -> Self {
        Self { regex_path, param_names_in_order }
    }

    pub fn set_regex_path(&mut self, regex_path: String) {
        self.regex_path = regex_path;
    }
    pub fn set_param_names_in_order(&mut self, param_names_in_order: Vec<String>) {
        self.param_names_in_order = param_names_in_order;
    }
}

/// `RouteHandlerContainer` is a struct that contains a `HashMap` of `Regex` and `HandlerFunction`
/// `Regex` is a semantic struct that contains a `String` field.
///
/// Properties:
///
/// * `map`: This is a HashMap that will store the regular expression and the handler function.
pub struct RouteHandlerContainer {
    str_regex_map: HashMap<String, RoutePath>,
    path_map: HashMap<RoutePath, HandlerFunction>,
}

impl Debug for RouteHandlerContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let key_vec: Vec<&RoutePath> = self.path_map.iter().map(|(k, _v)| {
            k
        }).collect();
        f.debug_struct("RouteHandlerContainer")
            .field("path_map", &key_vec)
            .finish()
    }
}

impl Providable for RouteHandlerContainer { }

impl RouteHandlerContainer {
    pub fn new() -> Self {
        Self { str_regex_map: HashMap::new(), path_map: HashMap::new() }
    }

    pub fn path_match(&self) {
        // desc: "/cars/{car_id}/get", getCar(), real: "/cars/2/get", getCar(), regex: "\/cars\/.{1,}\/get"
        // We get request path "/cars/2/get", we match it against regexes, "\/cars\/.{1,}\/get", *successful match*,
        // We then get and object containing "/cars/{car_id}/get", and a map of indexes and key names. e.g { 1, "car_id" },
        // We then split "/cars/2/get", and get the second particle, i.e "2", and map it to key value "car_id".

    }

    pub fn get(&self, path: &str) -> Option<&HandlerFunction> {
        let a = self.str_regex_map.get(path).map(|rp| {
            self.path_map.get(&rp)
        });

        match a {
            Some(t) => { t },
            None => { None }
        }
    }

    pub fn insert(&mut self, k: &str, v: HandlerFunction) {
        // e.g "/cars/{car_id}/get" to regex "\/cars\/.{1,}\/get"
        let mut k: String = String::from(k);
        let mut close_rurly_pos_vec: Vec<usize> = Vec::new();
        let mut open_curly_pos_vec: Vec<usize> = Vec::new();
        for (i, c) in k.chars().enumerate() {
            if c == '{' { open_curly_pos_vec.push(i); }
            else if c == '}' { close_rurly_pos_vec.push(i); }
        }
        if close_rurly_pos_vec.len() != open_curly_pos_vec.len() {
            dbg!(close_rurly_pos_vec.len(), open_curly_pos_vec.len(), k);
            panic!("close_rurly_pos_vec.len() != open_curly_pos_vec.len()");
        }
        for (open_curly_pos, close_curly_pos)
        in open_curly_pos_vec.iter().zip(close_rurly_pos_vec) {
            let key_name: String;
            
            k.replace_range(open_curly_pos..&(close_curly_pos+1), ".{1,}");
        }
        self.path_map.insert(k, v);
    }
}

#[cfg(test)]
mod test {
    use crate::web::server::data::models::transaction::Transaction;
    use crate::web::server::function_chain::route_handler_container::RouteHandlerContainer;

    fn dummy(_t: &mut Transaction) { }

    #[test]
    fn test() {
        let mut rhc = RouteHandlerContainer::new();
        rhc.insert("/hey/test", dummy);
        rhc.insert("/hey/{param}/test", dummy);

        dbg!(rhc);
    }
}
