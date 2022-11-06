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
pub enum JsonVariant {
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

/// `JsonObject` is a `HashMap` of `String`s to `JsonVariant`s.
///
/// Properties:
///
/// * `map`: This is the HashMap that will store the key-value pairs of the JSON object.
#[derive(Debug, Clone)]
pub struct JsonObject {
    map: HashMap<String, JsonVariant>,
}

/// Creating a custom error type for the json parser.
#[derive(Debug)]
pub enum JsonParseError {
    Error(String),
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        JsonVariant::JsonObject(self.to_owned()).to_string()
    }
}

#[derive(Debug)]
pub enum JsonGetError {
    InvalidType,
    NotFound
}

impl JsonObject {
    pub fn get(&self, k: &str) -> Option<&JsonVariant> {
        self.map.get(k)
    }

    /// Returns the value at the key as a array.
    /// If the key is not found, this will return a JsonGetError::NotFound,
    /// If the key is not referencing a array, it will return a JsonGetError::InvalidType
    ///
    /// Arguments:
    ///
    /// * `k`: &str - The key to get the value of.
    ///
    /// Returns:
    ///
    /// A reference to a Vec<JsonVariant>.
    pub fn get_array(&self, k: &str) -> Result<&Vec<JsonVariant>, JsonGetError> {
        if let JsonVariant::JsonArray(obj) = self.get_as_result(k)? {
            Ok(obj)
        } else {
            Err(JsonGetError::InvalidType)
        }
    }

    /// Returns the value at the key as a object.
    /// If the key is not found, this will return a JsonGetError::NotFound,
    /// If the key is not referencing a object, it will return a JsonGetError::InvalidType
    ///
    /// Arguments:
    ///
    /// * `k`: &str - The key to get the value of.
    ///
    /// Returns:
    ///
    /// A reference to a JsonObject.
    pub fn get_object(&self, k: &str) -> Result<&JsonObject, JsonGetError> {
        if let JsonVariant::JsonObject(obj) = self.get_as_result(k)? {
            Ok(obj)
        } else {
            Err(JsonGetError::InvalidType)
        }
    }

    /// Returns the value at the key as a string.
    /// If the key is not found, this will return a JsonGetError::NotFound,
    /// If the key is not referencing a string, it will return a JsonGetError::InvalidType
    ///
    /// Arguments:
    ///
    /// * `k`: &str - The key to get the value of.
    ///
    /// Returns:
    ///
    /// A reference to a String.
    pub fn get_string(&self, k: &str) -> Result<&String, JsonGetError> {
        if let JsonVariant::JsonString(str) = self.get_as_result(k)? {
            Ok(str)
        } else {
            Err(JsonGetError::InvalidType)
        }
    }

    /// If the key exists, return the value, otherwise return an error
    ///
    /// Arguments:
    ///
    /// * `k`: &str - The key to get the value of.
    ///
    /// Returns:
    ///
    /// A reference to a JsonVariant.
    fn get_as_result(&self, k: &str) -> Result<&JsonVariant, JsonGetError> {
        if let Some(json_variant) = self.get(k) {
            Ok(json_variant)
        } else {
            Err(JsonGetError::NotFound)
        }
    }

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

    /// Parses a byte vector slice representing the value in a key value pair.
    ///
    /// Arguments:
    ///
    /// * `val`: &[u8] - The value to parse
    ///
    /// Returns:
    ///
    /// A JsonVariant enum
    fn parse_value(val: &[u8]) -> Result<JsonVariant, JsonParseError> {
        match val[0] {
            123 => {
                let val: JsonObject = Self::parse_object(val)?;
                Ok(JsonVariant::JsonObject(val))
            },
            91 => {
                let split: Vec<Vec<u8>> = Self::split_by_element(val.to_vec());
                let mut result: Vec<JsonVariant> = Vec::new();
                for value in split {
                    match value[0] {
                        123 | 91 => {
                            result.push(Self::parse_value(value.as_slice())?);
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

    /// It takes a slice of bytes, trims it, splits it into a vector of slices of bytes, filters out
    /// empty slices, splits each slice into a key and a value, removes the quotation marks from the
    /// key, parses the value, and inserts the key and value into a hashmap
    ///
    /// Arguments:
    ///
    /// * `arr`: &[u8] - The array of bytes to parse
    ///
    /// Returns:
    ///
    /// A JsonObject
    pub fn parse_object(arr: &[u8]) -> Result<Self, JsonParseError> {
        let mut it: JsonObject = Self { map: HashMap::new() };
        let trimmed: Vec<u8> = Self::surgical_trim(arr);
        let mut split: Vec<Vec<u8>> = Self::split_by_element(trimmed);
        split = split.into_iter().filter(|b: &Vec<u8>| {
            !b.is_empty()
        }).collect::<Vec<Vec<u8>>>();
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

    /// It splits the array by commas, but only when the comma is not inside a string or array
    ///
    /// Arguments:
    ///
    /// * `arr`: &[u8] - The array to split
    ///
    /// Returns:
    ///
    /// A vector of slices of the original array.
    fn split_by_element<'a>(arr: Vec<u8>) -> Vec<Vec<u8>> {
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
        }).map(|a| a.to_vec())
            .collect::<Vec<Vec<u8>>>()
    }

    /// It removes all quotation marks that are not escaped
    ///
    /// Arguments:
    ///
    /// * `arr`: &[u8] - The array of bytes to remove the quotation marks from.
    ///
    /// Returns:
    ///
    /// A vector of bytes.
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

/// It takes a byte array and returns a JsonObject
///
/// Arguments:
///
/// * `bytes`: &[u8] - The bytes to parse into a JSON object.
///
/// Returns:
///
/// A Result<JsonObject, JsonParseError>
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
        dbg!(j_o.get_string("members").err());
        println!("String json: {}", JsonVariant::JsonObject(j_o).to_string());
        dbg!(Instant::now().duration_since(now));
    }
}