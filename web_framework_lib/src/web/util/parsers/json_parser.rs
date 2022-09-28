
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
        let mut vec: Vec<u8> = Self::remove_spaces_surgically(arr);
        vec = Self::slice_trailing_and_prefixing_curly_brackets(vec);
        let vec: Vec<&[u8]> = Self::split_top_level_properties_by_commas(&vec);
        let (key_vec, val_vec):
            (Vec<(&[u8], usize)>, Vec<(JsonVariant, Vec<u8>)>) = Self::extract_keys_and_values(vec);

        if log::log_enabled!(log::Level::Debug) {
            println!("key_vec_len = {}, val_vec_len = {}", key_vec.len(), val_vec.len());
        }
        if key_vec.len() != val_vec.len() {
            // TODO
            panic!("Bad format.")
        }
        Self {
            name: "hey".to_string()
        }
    }

    fn remove_spaces_surgically(arr: &[u8]) -> Vec<u8> {
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

    fn slice_trailing_and_prefixing_curly_brackets(vec: Vec<u8>) -> Vec<u8> {
        // Making sure the first character is "{" (123), and the last is "}" (125).
        if !(vec[0] == 123 && vec[vec.len()-1] == 125) {
            // TODO
            panic!("INVALID FIRST AND LAST BYTE");
        }
        // Slicing contents of outer curly brackets.
        vec[1..vec.len()-1].to_vec()
    }

    fn split_top_level_properties_by_commas<'a>(vec: &'a Vec<u8>) -> Vec<&'a [u8]> {
        let mut in_big_val: bool = false; // If we're in an array or object.
        vec.split(|byte: &u8| {
            // We split by commas "," if the comma is not inside of a value.
            if *byte == 91 || *byte == 123 { in_big_val = true; }
            else if *byte == 93 || *byte == 125 { in_big_val = false; }
            !in_big_val && *byte == 44
        })
            .collect::<Vec<&[u8]>>()
    }

    fn extract_keys_and_values<'a>(vec: Vec<&'a [u8]>) -> (Vec<(&'a [u8], usize)>, Vec<(JsonVariant, Vec<u8>)>) {
        let (mut key_vec, mut val_vec):
            (Vec<(&'a [u8], usize)>, Vec<(JsonVariant, Vec<u8>)>) = (Vec::new(), Vec::new());
        let len: usize = vec.iter().count();
        let _ = vec
            .into_iter()
            .map(|byte_slice: &[u8]| Self::split_line_into_keys_and_values(byte_slice)).enumerate()
            .map(|(i, (key, val)): (usize, (Option<&[u8]>, Option<&[u8]>))|
                Self::parse_line_key_value_split(key, val, &mut key_vec, &mut val_vec, len, i))
            .count();
        (key_vec, val_vec)
    }

    fn split_line_into_keys_and_values<'a>(byte_slice: &'a [u8]) -> (Option<&'a [u8]>, Option<&'a [u8]>) {
        if byte_slice[0] == 123 || byte_slice[0] == 91 {
            return (None, Some(byte_slice))
        }
        let index: Option<usize> = byte_slice.iter().position(|byte: &u8| *byte == 58);
        match index {
            Some(index) => {
                let colon_slice: (&[u8], &[u8]) = byte_slice.split_at(index);
                (Some(colon_slice.0), Some(&colon_slice.1[1..colon_slice.1.len()]))
            },
            None => { (None, Some(&byte_slice[1..byte_slice.len()])) }
        }
    }

    fn parse_line_key_value_split<'a>(key: Option<&'a [u8]>,
                                      val: Option<&[u8]>,
                                      key_vec: &mut Vec<(&'a [u8], usize)>,
                                      val_vec: &mut Vec<(JsonVariant, Vec<u8>)>,
                                      len: usize, i: usize) {
        let mut current: usize = 0;
        let mut val_buffer: Vec<u8> = Vec::new();
        let mut flush: bool = false;
        key.map(|k: &[u8]| {
            flush = !val_buffer.is_empty();
            current = val_vec.len();
            key_vec.push((k, current));
            val_buffer.clear();
        });
        // TODO: Put in function.
        if log::log_enabled!(log::Level::Debug) {
            key.map(|b: &[u8]| {
                println!("{}: Json key: {}", current, String::from_utf8_lossy(b));
            });
            val.map(|b: &[u8]| {
                println!("{}: Json val: {}", current, String::from_utf8_lossy(b));
            });
        }
        val.map(|v: &[u8]| {
            flush = !(v[0] == 123 || v[0] == 91);
            val_buffer.append(&mut v.to_vec());
        });
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
    }
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