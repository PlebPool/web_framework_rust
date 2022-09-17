use proc_macros_derive::EnumFromStr;
use proc_macros_derive::StrFromEnum;

#[derive(EnumFromStr, StrFromEnum, Eq, PartialEq, Hash, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}