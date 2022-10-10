

// Binary    Hex          Comments
// 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
// 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
// 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
// 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
// 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding

use std::collections::HashMap;
use std::io::BufRead;

//  █     █░▓█████   ▄████  ▄▄▄▄    ██▓     ▄▄▄      ▓█████▄
// ▓█░ █ ░█░▓█   ▀  ██▒ ▀█▒▓█████▄ ▓██▒    ▒████▄    ▒██▀ ██▌
// ▒█░ █ ░█ ▒███   ▒██░▄▄▄░▒██▒ ▄██▒██░    ▒██  ▀█▄  ░██   █▌
// ░█░ █ ░█ ▒▓█  ▄ ░▓█  ██▓▒██░█▀  ▒██░    ░██▄▄▄▄██ ░▓█▄   ▌
// ░░██▒██▓ ░▒████▒░▒▓███▀▒░▓█  ▀█▓░██████▒ ▓█   ▓██▒░▒████▓
// ░ ▓░▒ ▒  ░░ ▒░ ░ ░▒   ▒ ░▒▓███▀▒░ ▒░▓  ░ ▒▒   ▓▒█░ ▒▒▓  ▒
//   ▒ ░ ░   ░ ░  ░  ░   ░ ▒░▒   ░ ░ ░ ▒  ░  ▒   ▒▒ ░ ░ ▒  ▒
//   ░   ░     ░   ░ ░   ░  ░    ░   ░ ░     ░   ▒    ░ ░  ░
//     ░       ░  ░      ░  ░          ░  ░      ░  ░   ░
//                               ░                    ░

#[derive(Debug)]
enum JsonVariant {
    JsonObject(String, JsonObject),
    JsonArray(String, Vec<String>),
    JsonValue(String, String),
    None
}

#[derive(Debug)]
struct JsonObject {
    map: HashMap<String, JsonVariant>,
}

impl JsonObject {

    /// We trim the array, removing trailing and prefixing spaces. Then we turn it into an iterator. We
    /// deref every element once. We remove spaces outside of keys and vals. Also newlines. We collect
    /// result in an array
    ///
    /// Arguments:
    ///
    /// * `arr`: &[u8] -> The array we want to trim.
    ///
    /// Returns:
    ///
    /// A vector of bytes.
    fn surgical_trim(arr: &[u8]) -> Vec<u8> {
        let mut in_val: bool = false;
        // We trim the array, removing trailing and prefixing spaces.
        let arr: Vec<u8> = Self::trim_byte_array(arr)
            .into_iter() // Turn it into an iterator.
            .map(|b: &u8| *b) // We deref every element once.
            .filter(|byte: &u8| { // we remove spaces outside of keys and vals. Also newlines.
                if *byte == 34 { in_val = !in_val; } // 34
                !(!in_val && (*byte == 32 || *byte == 10))
            })
            .collect::<Vec<u8>>(); // We collect result in an array.
        arr
    }

    /// It takes a byte array and returns a byte array that is trimmed of leading and trailing
    /// whitespace
    ///
    /// Arguments:
    ///
    /// * `arr`: &[u8] - The array of bytes to trim.
    ///
    /// Returns:
    ///
    /// A slice of the original array.
    fn trim_byte_array(arr: &[u8]) -> &[u8] {
        if arr.len() > 0 {
            let (mut i, mut j): (usize, usize) = (0, arr.len());
            loop {
                if arr[i] != 32 && arr[j - 1] != 32 { break; }
                else {
                    if arr[i] == 32 { i = i + 1; }
                    if arr[j - 1] == 32 { j = j - 1; }
                }
            }
            &arr[i..j]
        } else { arr }
    }

    fn parse_array(slice: &[u8]) -> Vec<u8> {

        Vec::new()
    }

    fn private_parse(&mut self, vec: Vec<u8>) {
        let mut depth: isize = -1;
        let comma_split: Vec<&[u8]> = vec.split(|b: &u8| {
            match *b {
                123 | 91 => { depth = depth + 1; },
                125 | 93 => { depth = depth - 1; },
                _ => {  }
            }
            depth == 0 && *b == 44
        }).collect::<Vec<&[u8]>>();
        let key_val_tuples: Vec<(&[u8], &[u8])> = comma_split
            .into_iter()
            .map(|key_val_pair: &[u8]| {
            if let Some(index) =
            key_val_pair.iter().position(|b: &u8| { *b == 58 }) {
                key_val_pair.split_at(index)
            } else {
                // TODO
                panic!("NO SPLIT");
            }
        }).collect::<Vec<(&[u8], &[u8])>>();
        for (key, val) in key_val_tuples {
            match val[0] {
                91 => {
                    let array = Self::parse_array(val);
                    // Json array.
                },
                123 => {
                    let mut object = JsonObject::new();
                    object.parse(val);
                    // Json object.
                },
                _ => {  }
            }

        }
    }

    pub fn parse(&mut self, arr: &[u8]) -> Self {
        let vec: Vec<u8> = Self::surgical_trim(arr);
        let vec: Vec<u8> = vec[1..vec.len()-1].to_vec();
        dbg!(Self::private_parse(self, vec));
        return Self { map: Default::default() };
    }
    
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
}

pub fn parse_into_json_objects(s: &str) {
    let mut a = JsonObject::new();
    a.parse(s.as_bytes());
}

#[cfg(test)]
mod test {
    use std::env;
    use std::time::Instant;
    use crate::web::util::parsers::json_parser::parse_into_json_objects;

    const TEST_STR: &str = r#"
        {
        "squadName": "Super hero squad",
  "homeTown": "Metro City",
  "formed": 2016,
  "secretBase": "Super tower",
  "active": true,
  "members": [
    {
      "name": "Molecule Man",
      "age": 29,
      "secretIdentity": "Dan Jukes",
      "powers": ["Radiation resistance", "Turning tiny", "Radiation blast"],
    },
    {
      "name": "Madame Uppercut",
      "age": 39,
      "secretIdentity": "Jane Wilson",
      "powers": [
        "Million tonne punch",
        "Damage resistance",
        "Superhuman reflexes"
      ]
    },
    {
      "name": "Eternal Flame",
      "age": 1000000,
      "secretIdentity": "Unknown",
      "powers": [
        "Immortality",
        "Heat Immunity",
        "Inferno",
        "Teleportation",
        "Interdimensional travel"
      ]
    }
  ]
}
    "#;

    #[test]
    fn hey() {
        static RUST_LOG: &str = "RUST_LOG";
        static DEBUG: &str = "debug";
        env::set_var(RUST_LOG, DEBUG);
        let _ = env_logger::try_init();
        let now = Instant::now();
        parse_into_json_objects(TEST_STR);
        dbg!(Instant::now().duration_since(now));
    }
}