use std::collections::HashMap;
use std::io::Read;

// #[derive(Debug)]
// enum ProcessingModes {
//     ObjectStart,
//     ObjectEnd,
//     KeyValDelimiter,
//     ArrayStart,
//     ArrayEnd,
//     KeyOrValStartOrEnd,
//     KeyValSwitch,
//     EmptySpace,
//     NonReservedCharacter
// }

enum JsonVariant {
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    JsonValue(JsonValue)
}

struct JsonValue {
    name: String,
    value: String
}

impl JsonValue {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

struct TopJsonObject {
    map: HashMap<String, JsonVariant> // Values, objects, arrays.
}

impl TopJsonObject {
    pub fn new(map: HashMap<String, JsonVariant>) -> Self {
        Self { map }
    }
}

struct JsonObject {
    name: String,
    map: HashMap<String, JsonVariant> // Values, objects, arrays.
}

impl JsonObject {
    pub fn new(name: String, map: HashMap<String, JsonVariant>) -> Self {
        Self { name, map }
    }
}

struct JsonArray {
    name: String,
    array: Vec<JsonVariant> // Values, objects, arrays.
}

impl JsonArray {
    pub fn new(name: String, array: Vec<JsonVariant>) -> Self {
        Self { name, array }
    }
}

// Binary    Hex          Comments
// 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
// 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
// 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
// 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
// 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding
pub fn parse_into_json_objects(s: &str) {
    // We want to create a jsonObject with key value pairs, some values are objects, some
    // are arrays, some are strings or numbers, objects can be nested. Arrays contain 0 or more
    // objects.
    // Map top level keys / values,
    // go into values that are objects or arrays and handle them, turn them into objects that are
    // to be values.

    let bytes: Vec<u8> = s.trim()
        .replace("[", "[,")
        .replace("]", ",]")
        .replace("{", "{,")
        .replace("}", ",}")
        .bytes()
        .collect::<Vec<u8>>()
        .split(|b: &u8| *b == 10)
        .collect::<Vec<&[u8]>>()
        .into_iter()
        .map(|arr: &[u8]| Vec::from(trim_byte_array(arr)))
        .reduce(|mut accum: Vec<u8>, mut element: Vec<u8>| { accum.append(&mut element); accum })
        .expect("EMPTY");
    if bytes[0] != 123 || bytes[bytes.len() - 1] != 125 {
        panic!("Invalid first or last character. first: {}, last: {}", bytes[0], bytes[bytes.len() - 1]);
    } else {
        let _bytes = bytes[1..bytes.len() - 2].to_vec()
            .split(|b: &u8| *b == 44)
            .filter(|b: &&[u8]| b.len() != 0)
            .map(|b: &[u8]| {
                let split = b.split(|b: &u8| *b == 58).collect::<Vec<&[u8]>>();
                let len = split.len();
                let mut i: i32 = 0;
                let _ = split.into_iter()
                    .map(|part: &[u8]| {
                        let mut classification: &str = "Key";
                        if len == 1 || i % 2 != 0 { classification = "Val"; }
                        if trim_byte_array(part)[0] == 91 {
                            classification = "Array Start";
                            i = i - 1;
                        } else if trim_byte_array(part)[0] == 93 {
                            classification = "Array End";
                            i = i - 1;
                        }
                        else if trim_byte_array(part)[0] == 123 {
                            classification = "Object Start";
                            i = i - 1;
                        } else if trim_byte_array(part)[0] == 125 {
                            classification = "Object End";
                            i = i - 1;
                        }
                        println!("{}: {}", classification, String::from_utf8_lossy(part));
                        i = i + 1;
                    }).count();
            }).count();
    }
}

/// It takes a byte array and returns a byte array that is the same as the input array, but with all
/// leading and trailing whitespace removed
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

#[cfg(test)]
mod test {
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
      "powers": ["Radiation resistance", "Turning tiny", "Radiation blast"]
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
        dbg!(parse_into_json_objects(TEST_STR));
    }
}