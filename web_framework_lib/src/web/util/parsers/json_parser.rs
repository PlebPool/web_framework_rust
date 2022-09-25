use std::collections::HashMap;

enum JsonVariant {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
}

struct JsonParser {
    map: HashMap<String, JsonVariant>
}

struct JsonObject {
    map: HashMap<String, JsonVariant>
}

struct JsonArray {
    vec: Vec<JsonVariant>
}

struct JsonString {
    val: String
}