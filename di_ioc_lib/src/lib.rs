
pub mod di {
    pub mod ioc_container;
    pub mod provider_trait;
    pub mod providable_trait;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
