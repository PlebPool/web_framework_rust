

pub mod web {
    pub mod model {
        pub mod request;
        pub mod http_headers;
        pub mod response;
    }
    pub mod request_handling {
        pub mod request_handler;
    }
    pub mod util {
        pub mod encoders {
            pub mod url_encoder;
        }
        pub mod enums {
            pub mod http_method_enum;
            pub mod mime_types;
        }
        pub mod parsers {
            pub mod json_parser;
            pub mod request_parser;
        }
    }
    pub mod server;
}
