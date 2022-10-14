use std::collections::HashMap;
use std::ops::Add;

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

#[derive(Debug, Clone)]
enum JsonVariant {
    JsonObject(JsonObject),
    JsonArray(Vec<JsonVariant>),
    JsonString(String),
}

impl ToString for JsonVariant {
    fn to_string(&self) -> String {
        match self {
            JsonVariant::JsonObject(obj) => {
                let mut string: String = String::from("{");
                for (key, variant) in &obj.map {
                    string = string
                        .add(&format!(r#""{}""#, key))
                        .add(":")
                        .add(&variant.to_string())
                        .add(",");
                }
                string.add("}")
            },
            JsonVariant::JsonArray(vec) => {
                let mut string: String = String::from("[");
                for current in vec {
                    string = string.add(&current.to_string()).add(",");
                }
                string = string.add("]");
                string
            }
            JsonVariant::JsonString(s) => {
                String::from(format!(r#""{}""#, s))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonObject {
    map: HashMap<String, JsonVariant>,
}

#[derive(Debug)]
pub enum JsonParseError {
    Error(String),
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        JsonVariant::JsonObject(self.to_owned()).to_string()
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

    fn parse_value(val: &[u8]) -> Result<JsonVariant, JsonParseError> {
        match val[0] {
            123 => {
                let val: JsonObject = Self::parse_object(val)?;
                Ok(JsonVariant::JsonObject(val))
            },
            91 => {
                let split: Vec<&[u8]> = Self::split_by_element(val);
                let mut result: Vec<JsonVariant> = Vec::new();
                for value in split {
                    match value[0] {
                        123 | 91 => {
                            result.push(Self::parse_value(value)?);
                        },
                        _ => {
                            let a: Vec<u8> = value[1..value.len()-1].to_vec();
                            result.push(JsonVariant::JsonString(String::from_utf8(a).unwrap()));
                        }
                    }
                }
                Ok(JsonVariant::JsonArray(result))
            },
            _ => {
                let val: Vec<u8> = Self::remove_unescaped_quotation_marks(val);
                if let Ok(res) = String::from_utf8(val.to_vec()) {
                    Ok(JsonVariant::JsonString(res))
                } else {
                    Err(JsonParseError::Error(String::from("Failed to parse value to string")))
                }
            }
        }
    }

    pub fn parse_object(arr: &[u8]) -> Result<Self, JsonParseError> {
        let mut it: JsonObject = Self { map: HashMap::new() };
        let trimmed: Vec<u8> = Self::surgical_trim(arr);
        let mut split: Vec<&[u8]> = Self::split_by_element(trimmed.as_slice());
        split = split.into_iter().filter(|b: &&[u8]| {
            !b.is_empty()
        }).collect::<Vec<&[u8]>>();
        for current in split {
            let opt_pos: Option<usize> = current.iter().position(|b| *b == 58);
            if let Some(index) = opt_pos {
                let (key, mut val): (&[u8], &[u8]) = current.split_at(index);
                val = &val[1..val.len()];
                let key: String = String::from_utf8(Self::remove_unescaped_quotation_marks(key)).unwrap();
                let val: JsonVariant = Self::parse_value(val)?;
                it.map.insert(key, val);
            } else {
                return Err(JsonParseError::Error(String::from("No Colon")));
            }
        }
        Ok(it)
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
        })
            .collect::<Vec<&[u8]>>()
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

pub fn parse_into_json_object(bytes: &[u8]) -> Result<JsonObject, JsonParseError> {
    let json: Result<JsonObject, JsonParseError> = JsonObject::parse_object(bytes);
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("Parsed Json: {:?}", json)
    }
    json
}

#[cfg(test)]
mod test {
    use std::env;
    use std::time::Instant;
    use crate::web::util::parsers::json_parser::{JsonVariant, parse_into_json_object};

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
        let j_o = parse_into_json_object(TEST_STR.as_bytes()).unwrap();
        println!("String json: {}", JsonVariant::JsonObject(j_o).to_string());
        dbg!(Instant::now().duration_since(now));
    }
}