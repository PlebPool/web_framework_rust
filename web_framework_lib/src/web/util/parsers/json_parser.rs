use std::collections::HashMap;

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
    let str_as_bytes: Vec<u8> = s.trim().bytes().collect::<Vec<u8>>();
    let bytes: Vec<&[u8]> = str_as_bytes
        .split(|b: &u8| b == &10) // Splitting by newline.
        .collect::<Vec<&[u8]>>() // Collecting to vec of byte arrays.
        .into_iter() // Consuming iterator.
        .map(|arr: &[u8]| { // Trimming (removing trailing and prefixing spaces)
            let (mut i, mut j): (usize, usize) = (0, arr.len());
            loop {
                if arr[i] != 32 && arr[j - 1] != 32 { break; }
                else {
                    if arr[i] == 32 { i = i + 1; }
                    if arr[j - 1] == 32 { j = j - 1; }
                }
            }
            &arr[i..j]
        }).collect(); // Collecting result into final array.
    for a in bytes {
        dbg!(String::from_utf8_lossy(a));
    }
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