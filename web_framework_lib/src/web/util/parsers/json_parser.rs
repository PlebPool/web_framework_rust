#[derive(Debug)]
enum ProcessingModes {
    ObjectStart,
    ObjectEnd,
    KeyValDelimiter,
    ArrayStart,
    ArrayEnd,
    KeyOrValStartOrEnd,
    KeyValSwitch,
    EmptySpace,
    NonReservedCharacter
}

// Binary    Hex          Comments
// 0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
// 10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
// 110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
// 1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
// 11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding
pub fn parse_into_json_objects(s: &str) {
    dbg!(s);
    let trimmed: &str = s.trim();
    let as_bytes: Vec<u8> = trimmed.bytes().collect();
    let mut depth_index = 0;
    for b in as_bytes {
        let switch: ProcessingModes = match b {
            123 => { depth_index = depth_index + 1; ProcessingModes::ObjectStart },
            125 => { depth_index = depth_index - 1; ProcessingModes::ObjectEnd },
            91 => { depth_index = depth_index + 1; ProcessingModes::ArrayStart  },
            93 => { depth_index = depth_index - 1; ProcessingModes::ArrayEnd },
            44 => { ProcessingModes::KeyValDelimiter },
            34 => { ProcessingModes::KeyOrValStartOrEnd },
            58 => { ProcessingModes::KeyValSwitch }
            32 => { ProcessingModes::EmptySpace }
            _ => { ProcessingModes::NonReservedCharacter }
        };
        if b != 32 && b != 10 {
            println!("char: {}, switch: {:#?}, depth_index: {}", char::from(b), switch, depth_index);
        }
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