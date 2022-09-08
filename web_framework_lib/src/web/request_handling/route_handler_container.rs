use di_ioc_lib::di::providable_trait::Providable;
use std::collections::HashMap;
use std::ops::Add;
use regex::{Error, Regex};
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

// TODO: If global IoC is implemented, maybe make a macro that makes it easier to match handlers, and paths.
impl RouteHandlerContainer {
    pub fn new() -> Self {
        Self { path_map: HashMap::new() }
    }

    /// "/cars/{car_id}/wow/"
    /// "/cars/2/wow/" maybe split by slashes and match them?
    pub fn get_match(&self, path: &str) -> Option<&HandlerFunction> {
        self.path_map.iter().find(|(regex_str, _)| {
            let reg_match_result: Result<bool, Error> = Regex::new(regex_str).map(|regex_struct: Regex| {
                let val: bool = regex_struct.is_match(&path);
                // dbg!(&regex_struct, &path, &val);
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!("\n regex: {},\n path: {},\n is_match: {}", regex_str, path, val);
                }
                val
            });
            return match reg_match_result {
                Err(error) => {
                    log::error!("{}", error);
                    false
                },
                Ok(t) => { t }
            };
        }).map(|(_, h): (_, &HandlerFunction)| h)
    }

    /// It takes a string and a function, and inserts the function into a hashmap, where the key is a
    /// regex pattern that matches the string
    /// e.g
    /// ```
    /// let mut rhc = RouteHandlerContainer::new();
    /// rhc.insert("/hey/test", dummy);
    /// rhc.insert("/hey/{param}/test", dummy);
    /// ```
    /// The string within {} does not matter, it's primarily for semantics.
    /// Any path cell "/cell/" that contains "{ }" will be replaced with ".{1,}"
    /// Arguments:
    ///
    /// * `k`: &str, v: HandlerFunction
    /// * `v`: HandlerFunction
    pub fn insert(&mut self, k: &str, v: HandlerFunction) {
        let mut k: String = String::from(k);
        let mut closed_curly_brackets_pos_vec: Vec<usize> = Vec::new();
        let mut open_curly_brackets_pos_vec: Vec<usize> = Vec::new();
        for (i, c) in k.chars().enumerate() {
            if c == '{' {
                open_curly_brackets_pos_vec.push(i);
            } else if c == '}' {
                closed_curly_brackets_pos_vec.push(i);
            }
        }
        if open_curly_brackets_pos_vec.len() != closed_curly_brackets_pos_vec.len() {
            // dbg!(open_curly_brackets_pos_vec.len(), closed_curly_brackets_pos_vec.len(), k);
            if log::log_enabled!(log::Level::Error) {
                log::error!("open_curly_pos_vec.len(): {}, closed_curly_brackets_pos_vec.len(): {}, k: {}",
                    open_curly_brackets_pos_vec.len(),
                    closed_curly_brackets_pos_vec.len(),
                    k
                );
            }
            panic!("open_curly_brackets_pos_vec.len() != closed_curly_brackets_pos_vec.len()");
        }
        for (open, closed)
        in open_curly_brackets_pos_vec.iter().zip(closed_curly_brackets_pos_vec) {
            k.replace_range(open..&(closed+1), ".{1,}");
        }
        k = String::from("^").add(&k.add("$"));
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
        rhc.insert("/hey/test", dummy);
        rhc.insert("/hey/{param}/test", dummy);
    }
}