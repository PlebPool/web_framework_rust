
#[derive(Debug)]
struct JsonObject {
    name: String,
}
// Binary    Hex          Comments
// 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
// 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
// 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
// 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
// 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding

#[derive(Debug)]
enum JsonVariant {
    JsonObject,
    JsonArray,
    JsonValue,
}

impl JsonObject {
    pub fn parse(arr: &[u8]) -> Self {
        let mut in_val: bool = false;

        // We trim the array, removing trailing and prefixing spaces.
        let arr: Vec<u8> = trim_byte_array(arr)
            .into_iter() // Turn it into an iterator.
            .map(|b: &u8| *b) // We deref every element once.
            .filter(|byte: &u8| { // we remove spaces outside of keys and vals. Also newlines.
                if *byte == 34 { in_val = !in_val; } // 34
                !(!in_val && (*byte == 32 || *byte == 10))
            })
            .collect::<Vec<u8>>(); // We collect result in an array.
        // Because we need it in a variable for next step.

        // Making sure the first character is "{" (123), and the last is "}" (125).
        if !(arr[0] == 123 && arr[arr.len()-1] == 125) {
            panic!("INVALID FIRST AND LAST BYTE");
        }
        // Slicing contents of outer curly brackets.
        let arr: &[u8] = &arr[1..arr.len()-1];

        let mut in_big_val: bool = false; // If we're in an array or object.
        let arr: Vec<&[u8]> = arr.split(|byte: &u8| {
            // We split by commas "," if the comma is not inside of a value.
            if *byte == 91 || *byte == 123 {
                in_big_val = true;
            } else if *byte == 93 || *byte == 125 {
                in_big_val = false;
            }
            !in_big_val && *byte == 44
        })
            .collect::<Vec<&[u8]>>();
        // We collect result in array because we need it in a variable for the next step.

        // This vector holds all the keys, it also holds the index to the value. Which is in a seperate vector.
        let mut key_vec: Vec<(&[u8], usize)> = Vec::new();
        // This vector hold all the values, it also holds the JsonVariant of the value.
        let mut val_vec: Vec<(JsonVariant, Vec<u8>)> = Vec::new();
        // Temporary buffer used for multiline values like arrays and objects that need to concatonated.
        let mut val_buffer: Vec<u8> = Vec::new();
        // The current index, this is needed as we're synchronizing the indexes of the key and val vectors.
        let mut current: usize = 0;
        // Needed for EOF flushing of the buffer.
        let len: usize = arr.iter().count();

        let _ = arr
            .into_iter()
            .map(|byte_slice: &[u8]| {
                // This map returns a tuple containing either (key, val) or (None, val).
                // Depending on if the line we're processing holds a key or not.
                // If it returns (None, val) we concat the value onto the previous one.
                // This is why we have the "val_buffer" above.
                if byte_slice[0] == 123 || byte_slice[0] == 91 {
                    // If it starts with either 123 or 91, then the line is a a part of a
                    // multiproperty value like an array or and object.
                    return (None, Some(byte_slice))
                }
                // Getting the index of possible key val delimiter.
                let index: Option<usize> = byte_slice.iter().position(|byte: &u8| *byte == 58);
                // If we have an index, we have a key and a value.
                // If we don't, then we only have a value.
                match index {
                    Some(index) => {
                        let colon_slice: (&[u8], &[u8]) = byte_slice.split_at(index);
                        // The slice in the second element is to get rid of the colon,
                        // this is needed because split_at() is inclusive.
                        (Some(colon_slice.0), Some(&colon_slice.1[1..colon_slice.1.len()]))
                    },
                    None => {
                        // The slice in the second element is to get rid of the colon,
                        // this is needed because split_at() is inclusive.
                        (None, Some(&byte_slice[1..byte_slice.len()]))
                    }
                }
            })
            .enumerate() // For EOF flushing.
            .map(|(i, (key, val)): (usize, (Option<&[u8]>, Option<&[u8]>))| {
                let mut flush: bool = false; // Whether we flush buffer or not.
                key.map(|k: &[u8]| {
                    flush = !val_buffer.is_empty();
                    // Current index, we're not decrementing with 1, because the val_vec will grow,
                    // later on in this map instance.
                    current = val_vec.len();
                    // Pushing key and index of future value attributed to it.
                    key_vec.push((k, current));
                    // Clearing buffer.
                    val_buffer.clear();
                });
                if log::log_enabled!(log::Level::Debug) {
                    key.map(|b| {
                        println!("{}: Json key: {}", current, String::from_utf8_lossy(b));
                    });
                    val.map(|b: &[u8]| {
                        println!("{}: Json val: {}", current, String::from_utf8_lossy(b));
                    });
                }
                val.map(|v: &[u8]| {
                    // We're not flushing if the value starts with 123 or 91.
                    flush = !(v[0] == 123 || v[0] == 91);
                    val_buffer.append(&mut v.to_vec());
                });
                // We flush the buffer if flush is true or if we're on EOF.
                if flush || i == len - 1 {
                    if !val_buffer.is_empty() {
                        let variant: JsonVariant =
                            match val_buffer[1] {
                                123 => { JsonVariant::JsonObject },
                                91 => { JsonVariant::JsonArray },
                                _ => { JsonVariant::JsonValue }
                            };
                        val_vec.push((variant, val_buffer.to_owned()));
                    }
                }
            }).count(); // Triggering map.
        if log::log_enabled!(log::Level::Debug) {
            println!("key_vec_len = {}, val_vec_len = {}", key_vec.len(), val_vec.len());
        }
        Self {
            name: "hey".to_string()
        }
    }
}

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

pub fn parse_into_json_objects(s: &str) {
    let _ = JsonObject::parse(s.as_bytes());
}

#[cfg(test)]
mod test {
    use std::env;
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
        static RUST_LOG: &str = "RUST_LOG";
        static DEBUG: &str = "debug";
        env::set_var(RUST_LOG, DEBUG);
        let _ = env_logger::try_init();
        dbg!(parse_into_json_objects(TEST_STR));
    }
}