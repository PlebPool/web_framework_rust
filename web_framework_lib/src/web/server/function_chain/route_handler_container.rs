use std::collections::HashMap;
use di_ioc_lib::di::providable_trait::Providable;
use crate::web::server::HandlerFunction;

pub struct RouteHandlerContainer {
    map: HashMap<String, HandlerFunction>
}

impl Providable for RouteHandlerContainer { }

impl RouteHandlerContainer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn insert(&mut self, k: String, v: HandlerFunction) {
        self.map.insert(k, v);
    }

    pub fn remove(&mut self, k: &str) {
        self.map.remove(k);
    }

    pub fn get(&self, k: &str) -> Option<&HandlerFunction> {
        self.map.get(k)
    }
}