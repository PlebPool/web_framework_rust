

pub mod web {
    pub mod server;
}

pub mod application {
    pub mod di {
        pub mod container;
        pub mod providable_trait;
        pub mod provider_trait;
    }
    pub mod application_context;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}