extern crate core;

pub mod web {
    pub mod models {
        pub mod transaction;
    }
    pub mod request_handling {
        pub mod chain_handler;
        pub mod route_handler_container;
    }
    pub mod util {
        pub mod enums {
            pub mod http_method_enum;
            pub mod static_file_ext_enum;
        }
        pub mod request_parser;
    }
    pub mod server;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
