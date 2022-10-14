

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
#[derive(Debug, Clone, PartialEq, Eq)]
enum JsonPropertyToken {
    JsonObjectStart,
    JsonObjectEnd,
    JsonArrayStart,
    JsonArrayEnd,
    None
}

impl JsonPropertyToken {
    pub fn get_variant(&self) -> JsonVariant {
        match self {
            JsonPropertyToken::JsonObjectStart | JsonPropertyToken::JsonObjectEnd => {
                JsonVariant::JsonObject
            },
            JsonPropertyToken::JsonArrayStart | JsonPropertyToken::JsonArrayEnd => {
                JsonVariant::JsonArray
            },
            JsonPropertyToken::None => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug)]
enum JsonVariant {
    JsonObject,
    JsonArray,
    JsonValue,
    None
}

#[derive(Debug)]
struct JsonObject {
    map: HashMap<String, JsonObject>,
    top_level: bool
}

#[derive(Debug, PartialEq)]
struct JsonIndexSearchStruct {
    variant_token: JsonPropertyToken,
    index: usize,
    depth: usize,
    target_variant: JsonPropertyToken,
}

#[derive(Debug)]
struct JsonStartEnd {
    variant: JsonVariant,
    start: usize,
    end: usize,
    depth: usize,
}

impl JsonIndexSearchStruct {
    pub fn new(variant: JsonPropertyToken, index: usize, depth: usize, target_variant: JsonPropertyToken) -> Self {
        Self { variant_token: variant, index, depth, target_variant }
    }

    pub fn variant(&self) -> &JsonPropertyToken {
        &self.variant_token
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn target_variant(&self) -> &JsonPropertyToken {
        &self.target_variant
    }
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

    fn private_parse(vec: Vec<u8>) {
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
                    // Json array.
                },
                123 => {
                    // Json object.
                },
                _ => {  }
            }

        }


        //
        // let ignore: [u8; 1] = [34];
        // vec.reverse();
        // let mut key_buffer: Vec<u8> = Vec::new();
        // let mut val_buffer: Vec<u8> = Vec::new();
        // let mut key: bool = true;
        // let mut escaped: bool = false;
        // let mut depth: isize = -1;
        // if vec.len() < 1 {
        //     panic!("EMPTY");
        // }
        // for _ in 0..vec.len() {
        //     let current: u8 = vec.pop().expect("Could not get byte.");
        //     match current {
        //         123 | 91 => { depth = depth + 1; },
        //         125 | 93 => { depth = depth - 1; },
        //         92 => escaped = true,
        //         44 => {
        //             if depth == 0 {
        //                 break;
        //             }
        //         },
        //         58 => key = !key,
        //         _ => {
        //             if !escaped && ignore.contains(&current) { continue; }
        //             if key { key_buffer.push(current); }
        //             else { val_buffer.push(current); }
        //         }
        //     }
        // }
        // vec.reverse();
        // (String::from_utf8(key_buffer).unwrap(),
        //  String::from_utf8(val_buffer).unwrap())
    }

    pub fn parse(arr: &[u8]) -> Self {
        let vec = Self::surgical_trim(arr);
        let vec = vec[1..vec.len()-1].to_vec();
        dbg!(Self::private_parse(vec));
        return Self { map: Default::default(), top_level: true };
        let type_start_end_depth: Vec<JsonIndexSearchStruct>;
        {
            let vec: Vec<u8> = Self::surgical_trim(arr);
            let (mut depth, mut current_depth): (usize, usize) =  (0, 0);
            type_start_end_depth = vec
                .iter()
                .enumerate()
                .map(|(i, byte): (usize, &u8)| {
                    if *byte == 123 { (JsonPropertyToken::JsonObjectStart, i, true, JsonPropertyToken::JsonObjectEnd) }
                    else if *byte == 125 { (JsonPropertyToken::JsonObjectEnd, i, false, JsonPropertyToken::None) }
                    else if *byte == 91 { (JsonPropertyToken::JsonArrayStart, i, true, JsonPropertyToken::JsonArrayEnd) }
                    else if *byte == 93 { (JsonPropertyToken::JsonArrayEnd, i, false, JsonPropertyToken::None) }
                    else { (JsonPropertyToken::None, i, false, JsonPropertyToken::None) }
                })
                .filter(|(variant, _index, _start, _target_variant): &(JsonPropertyToken, usize, bool, JsonPropertyToken)| {
                    *variant != JsonPropertyToken::None
                })
                .map(|(variant, i, start, target_variant): (JsonPropertyToken, usize, bool, JsonPropertyToken)| {
                    if start { current_depth = current_depth + 1; }
                    depth = current_depth - 1;
                    if !start { current_depth = current_depth - 1; }
                    JsonIndexSearchStruct::new(variant, i, depth, target_variant)
                }).collect::<Vec<JsonIndexSearchStruct>>();
        }
        let json_start_end_structs: Vec<JsonStartEnd> = {
            let mut taken_indexes: Vec<usize> = Vec::new();
            type_start_end_depth.iter()
                .filter(|search_struct: &&JsonIndexSearchStruct| {
                    search_struct.target_variant != JsonPropertyToken::None
                })
                .map(|search_struct: &JsonIndexSearchStruct| {
                    let pos: Option<usize> = type_start_end_depth.iter().enumerate().position(|(i, it): (usize, &JsonIndexSearchStruct)| {
                        it.variant() == search_struct.target_variant() && it.depth() == search_struct.depth() && !taken_indexes.contains(&i)
                    });
                    if let Some(index) = pos {
                        if let Some(target) = type_start_end_depth.get(index) {
                            taken_indexes.push(index);
                            return JsonStartEnd {
                                variant: search_struct.variant_token.get_variant(),
                                start: index,
                                end: target.index,
                                depth: search_struct.depth
                            }
                        } else {
                            // TODO.
                            panic!("Failed to fetch target.");
                        }
                    } else {
                        // TODO.
                        panic!("Could not find index of valid target variant.");
                    }
                }).collect()
        };

        dbg!(json_start_end_structs);
        Self { map: Default::default(), top_level: true }
    }
}

pub fn parse_into_json_objects(s: &str) {
    let _ = JsonObject::parse(s.as_bytes());
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