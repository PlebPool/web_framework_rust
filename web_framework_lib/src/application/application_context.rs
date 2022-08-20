// use std::any::{Any, TypeId};
// use std::collections::HashMap;
//
// pub trait ContextAble {
//     fn constructor_for_context(props: FunctionProps) -> Self;
// }
//
// pub struct FunctionProps<'a> {
//     props: HashMap<TypeId, &'a dyn Any>
// }
//
// pub struct ApplicationContext<'a> {
//     constructor_context_map: HashMap<&'a str, fn(props: FunctionProps)>
// }
//
// impl <'a>ApplicationContext<'a> {
//     pub fn new(constructor_context_map: HashMap<&'a str, fn(FunctionProps)>) -> Self {
//         Self { constructor_context_map }
//     }
//
//     pub fn constructor_context_map(&self) -> &HashMap<&'a str, fn(FunctionProps)> {
//         &self.constructor_context_map
//     }
//
//     pub fn add_constructor_to_context(&mut self, name: &'a str, fn_p: fn(props: FunctionProps)) {
//         self.constructor_context_map.insert(name, fn_p);
//     }
// }
// TODO