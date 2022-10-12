

// Binary    Hex          Comments
// 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
// 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
// 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
// 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
// 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding

use std::collections::HashMap;
use std::io::BufRead;
use crate::web::util::parsers::json_parser::JsonVariant::{JsonObject as OtherJsonObject, JsonValue};

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
    JsonObject(JsonObject),
    JsonArray(Vec<JsonVariant>),
    JsonValue(String),
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

    fn parse_value(val: &[u8]) -> JsonVariant {
        match val[0] {
            123 => {
                let val: JsonObject = Self::parse_object(val);
                JsonVariant::JsonObject(val)
            },
            91 => {
                let split: Vec<&[u8]> = Self::split_by_element(val);
                let mut result: Vec<JsonVariant> = Vec::new();
                for value in split {
                    match value[0] {
                        123 | 91 => {
                            result.push(Self::parse_value(value));
                        },
                        _ => {
                            let a: Vec<u8> = value[1..value.len()-1].to_vec();
                            result.push(JsonVariant::JsonValue(String::from_utf8(a).unwrap()));
                        }
                    }
                }
                JsonVariant::JsonArray(result)
            },
            _ => {
                let val: Vec<u8> = Self::remove_unescaped_quotation_marks(val);
                JsonVariant::JsonValue(String::from_utf8(val.to_vec())
                        .expect("Failed val parse"))
            }
        }
    }

    pub fn parse_object(arr: &[u8]) -> Self {
        let mut it: JsonObject = Self { map: Default::default() };
        let trimmed: Vec<u8> = Self::surgical_trim(arr);
        let mut split: Vec<&[u8]> = Self::split_by_element(trimmed.as_slice());
        split = split.into_iter().filter(|b| {
            !b.is_empty()
        }).collect::<Vec<&[u8]>>();
        for current in split {
            let opt_pos: Option<usize> = current.iter().position(|b| *b == 58);
            if let Some(index) = opt_pos {
                let (key, mut val): (&[u8], &[u8]) = current.split_at(index);
                val = &val[1..val.len()];
                let key: String = String::from_utf8(Self::remove_unescaped_quotation_marks(key)).unwrap();
                let val = Self::parse_value(val);
                it.map.insert(key, val);
            } else {
                // TODO:
                panic!("NO COLON: {}", String::from_utf8_lossy(current));
            }
        }
        it
    }

    fn split_by_element(arr: &[u8]) -> Vec<&[u8]> {
        let arr: &[u8] = &arr[1..arr.len()-1];
        let mut depth: usize = 0;
        let mut switch: bool = true;
        arr.split(|b: &u8| {
            match *b {
                34 => {
                    if switch { depth = depth + 1; }
                    else { depth = depth - 1; }
                    switch = !switch;
                },
                123 | 91 => { depth = depth + 1; },
                125 | 93 => { depth = depth - 1; },
                _ => { }
            }
            depth == 0 && *b == 44
        }).collect::<Vec<&[u8]>>()
    }

    fn remove_unescaped_quotation_marks(arr: &[u8]) -> Vec<u8> {
        let mut escaped: bool = false;
        arr.into_iter().filter(|b| {
            if **b == 92 {
                escaped = true;
            }
            if escaped {
                escaped = false;
                return true
            } else {
                **b != 34
            }
        }).map(|b: &u8| *b).collect::<Vec<u8>>()
    }
}

pub fn parse_into_json_objects(s: &str) {
    dbg!(JsonObject::parse_object(s.as_bytes()));
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
  "amongus": {
    "sus": true,
  },
  "members": [
  "uwu",
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