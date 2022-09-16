use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RESERVED_AND_UNSAFE_CHARACTERS:
    HashMap<&'static str, &'static str> = initialize_map();
}

/// It creates a hashmap of all the characters that need to be escaped and their escaped counterparts
///
/// Returns:
///
/// A HashMap with the keys being the characters that need to be encoded and the values being the
/// encoded characters.
fn initialize_map() -> HashMap<&'static str, &'static str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert(":", "%3A");map.insert("/", "%2F");map.insert("?", "%3F");
    map.insert("#", "%23");map.insert("[", "%5B");map.insert("]", "%5D");
    map.insert("@", "%40");map.insert("!", "%21");map.insert("$", "%24");
    map.insert("&", "%26");map.insert("'", "%27");map.insert("(", "%28");
    map.insert(")", "%29");map.insert("*", "%2A");map.insert("+", "%2B");
    map.insert(",", "%2C");map.insert(";", "%3B");map.insert("=", "%3D");
    map.insert("%", "%25");map.insert(" ", "%20");map
}

/// It takes a string, and replaces all the reserved and unsafe characters with their corresponding safe
/// characters
///
/// Arguments:
///
/// * `encoded`: The string to be decoded.
///
/// Returns:
///
/// A String
pub fn decode(encoded: &str) -> String {
    let mut decoded: String = String::from(encoded);
    for (k, v) in RESERVED_AND_UNSAFE_CHARACTERS.iter() {
        decoded = decoded.replace(v, k);
    }
    decoded
}

/// It takes a string, replaces all the reserved and unsafe characters with their percent-encoded
/// equivalents, and returns the result
///
/// Arguments:
///
/// * `decoded`: The string to be encoded.
///
/// Returns:
///
/// A String
pub fn encode(decoded: &str) -> String {
    let mut encoded: String = String::from(decoded);
    for (k, v) in RESERVED_AND_UNSAFE_CHARACTERS.iter() {
        encoded = encoded.replace(k, v);
    }
    encoded
}