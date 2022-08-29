#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait EnumFromStr<T> {
    fn from_str(s: &str) -> Result<T, ()>;
}
