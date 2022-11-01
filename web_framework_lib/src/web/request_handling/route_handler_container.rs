use di_ioc_lib::di::providable_trait::Providable;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{LockResult, Mutex, MutexGuard};
use regex::{Error, Regex};
use crate::web::server::HandlerFunction;
use crate::web::util::enums::http_method_enum::HttpMethod;

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

/// `RouteHandlerContainer` is a struct that contains a `HashMap` of `Regex` and `HandlerFunction`
/// `Regex` is a semantic struct that contains a `String` field.
///
/// Properties:
///
/// * `map`: This is a HashMap that will store the regular expression and the handler function.
pub struct RouteHandlerContainer {
    method_map: Mutex<HashMap<HttpMethod, HashMap<String, HandlerFunction>>>,
}

impl Providable for RouteHandlerContainer { }

impl RouteHandlerContainer {
    pub fn new() -> Self {
        let mut map: HashMap<HttpMethod, HashMap<String, HandlerFunction>> = HashMap::new();
        map.insert(HttpMethod::GET, HashMap::new());
        map.insert(HttpMethod::POST, HashMap::new());
        map.insert(HttpMethod::PUT, HashMap::new());
        map.insert(HttpMethod::DELETE, HashMap::new());
        Self { method_map: Mutex::new(map) }
    }

    /// "/cars/{car_id}/wow/"
    /// "/cars/2/wow/" maybe split by slashes and match them?
    pub fn get_match(&self, path: &str, method: &HttpMethod) -> Option<HandlerFunction> {
        let lock_res: LockResult<MutexGuard<HashMap<HttpMethod, HashMap<String, HandlerFunction>>>> =
            self.method_map.lock();
        let method_map: MutexGuard<HashMap<HttpMethod, HashMap<String, HandlerFunction>>> =
            match lock_res {
                Ok(t) => {
                    t
                },
                Err(_) => {
                    unimplemented!();
                }
            };
        let path_map: Option<&HashMap<String, HandlerFunction>> = method_map.get(&method);
        if path_map.is_none() {
            return None;
        }
        let path_map: &HashMap<String, HandlerFunction> =
            path_map.expect("Failed to get path_map.");
        path_map.iter().find(|(regex_str, _)| {
            let reg_match_result: Result<bool, Error> = Regex
            ::new(regex_str).map(|regex_struct: Regex| {
                let val: bool = regex_struct.is_match(&path);
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
        }).map(|(_, h): (_, &HandlerFunction)| *h)
    }

    /// It takes a string and a function, and inserts the function into a hashmap, where the key is a
    /// regex pattern that matches the string
    /// e.g
    /// ```
    /// let mut rhc = RouteHandlerContainer::new();
    /// rhc.insert("/hey/test", dummy);
    /// rhc.insert("/hey/{param}/test", dummy);
    /// ```
    ///
    /// THREAD SAFE
    ///
    /// The string within {} does not matter, it's primarily for semantics.
    /// Any path cell "/cell/" that contains "{ }" will be replaced with ".{1,}"
    /// Arguments:
    ///
    /// * `k`: &str, v: HandlerFunction
    /// * `v`: HandlerFunction
    pub fn insert(&self, path: &str, handler_function: HandlerFunction, method: HttpMethod) {
        let mut k: String = String::from(path);
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
            if log::log_enabled!(log::Level::Error) {
                log::error!("open_curly_pos_vec.len(): {}, closed_curly_brackets_pos_vec.len(): {}, path: {}",
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
        let _ = self.method_map
            .lock()
            .map(|mut mutex_guard_map: MutexGuard<HashMap<HttpMethod, HashMap<String, HandlerFunction>>>| {
                mutex_guard_map
                    .get_mut(&method)
                    .map(|the_methods_map: &mut HashMap<String, HandlerFunction>| {
                        the_methods_map.insert(k, handler_function);
                    });
            });
    }
}

#[cfg(test)]
mod test {
    use crate::web::models::request::Request;
    use crate::web::models::response::Response;
    use crate::web::request_handling::route_handler_container::RouteHandlerContainer;
    use crate::web::util::enums::http_method_enum::HttpMethod;

    fn dummy<'a>(_t: &Request) -> Response<'a> {
        Response::not_found()
    }

    #[test]
    fn test() {
        let rhc = RouteHandlerContainer::new();
        rhc.insert("/hey/test", dummy, HttpMethod::GET);
        rhc.insert("/hey/{param}/test", dummy, HttpMethod::GET);
    }
}
