extern crate core;

pub mod web {
    pub mod models {
        pub mod transaction;
    }
    pub mod request_handling {
        pub mod request_handler;
        pub mod route_handler_container;
    }
    pub mod util {
        pub mod encoders {
            pub mod url_encoder;
        }
        pub mod enums {
            pub mod http_method_enum;
            pub mod static_file_ext_enum;
        }
        pub mod parsers {
            pub mod json_parser;
        }
        pub mod request_parser;
    }
    pub mod server;
}
