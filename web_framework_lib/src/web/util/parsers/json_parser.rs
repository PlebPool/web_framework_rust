
#[derive(Debug)]
struct JsonObject {
    name: String,
}

impl JsonObject {
    // Binary    Hex          Comments
    // 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
    // 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
    // 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
    // 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
    // 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding
    pub fn parse(arr: &[u8]) -> Self {
        let mut in_val: bool = false;
        let arr: Vec<u8> = trim_byte_array(arr)
            .into_iter()
            .map(|b: &u8| *b) // Deref
            .filter(|byte: &u8| {
                // Removing spaces (32) outside of keys and vals. We're also removing all newlines.
                if *byte == 34 { in_val = !in_val; } // 34
                !(!in_val && (*byte == 32 || *byte == 10))
            })
            .collect::<Vec<u8>>();
        // Making sure the first character is "{" (123), and the last is "}" (125).
        if !(arr[0] == 123 && arr[arr.len()-1] == 125) {
            panic!("INVALID FIRST AND LAST BYTE");
        }
        // Slicing contents of outer curly brackets.
        let arr: &[u8] = &arr[1..arr.len()-1];
        let _ = arr
            .split(|byte: &u8| *byte == 0x02C)// Splitting by comma "," (44)
            .map(|byte_slice: &[u8]| {
                // Getting index of delimiter colon ":" (58).
                let index: Option<usize> = byte_slice.iter().position(|byte: &u8| *byte == 58);
                // Returns tuple (Some(key), Some(val)), returns None for key if we're in an array.
                match index {
                    Some(index) => {
                        let colon_slice: (&[u8], &[u8]) = byte_slice.split_at(index);
                        (Some(colon_slice.0), Some(colon_slice.1))
                    },
                    None => { (None, Some(byte_slice)) }
                }
            })
            .map(|(key, val): (Option<&[u8]>, Option<&[u8]>)| {
                key.map(|key_slice: &[u8]| {
                    println!("KEY||| {}", String::from_utf8_lossy(key_slice));
                });
                val.map(|val_slice: &[u8]| {
                    println!("VAL||| {}", String::from_utf8_lossy(val_slice));
                });
            })
            .count(); // Triggering map.
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