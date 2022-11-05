use std::{collections::HashMap, io::Read};

#[derive(Debug)]
struct Input<T> {
    location: usize,
    input: Vec<T>,
}

impl<T> Input<T> {
    fn new(input: Vec<T>) -> Self {
        Self { location: 0, input }
    }
}

impl Input<char> {
    fn parse_char(&mut self, c: char) -> bool {
        if self.location + 1 > self.input.len() {
            return false;
        }

        if self.input[self.location] == c {
            self.location += 1;
            return true;
        }

        return false;
    }

    fn parse_str(&mut self, s: &str) -> bool {
        if self.location + s.len() > self.input.len() {
            return false;
        }

        if self.input[self.location..self.location + s.len()]
            .iter()
            .collect::<String>()
            == s
        {
            self.location += s.len();
            return true;
        }

        return false;
    }

    fn parse_while(&mut self, p: fn(char) -> bool) -> Option<String> {
        let mut idx = self.location;

        while idx < self.input.len() && p(self.input[idx]) {
            idx += 1;
        }

        if idx == self.location {
            return None;
        }

        let start = self.location;
        self.location = idx;
        return Some(self.input[start..self.location].iter().collect::<String>());
    }

    fn parse_whitespace(&mut self) {
        while self.location < self.input.len() && self.input[self.location].is_ascii_whitespace() {
            self.location += 1;
        }
    }
}

#[derive(Debug, PartialEq)]
enum JsonValue {
    JsonNull,
    JsonBool(bool),
    JsonNumber(usize),
    JsonString(String),
    JsonArray(Vec<JsonValue>),
    JsonObject(HashMap<String, JsonValue>),
}

impl Iterator for Input<char> {
    type Item = JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parse_str("null") {
            return Some(JsonValue::JsonNull);
        }
        if self.parse_str("true") {
            return Some(JsonValue::JsonBool(true));
        }
        if self.parse_str("false") {
            return Some(JsonValue::JsonBool(false));
        }
        if let Some(s) = self.parse_while(|c| c.is_ascii_digit()) {
            return Some(JsonValue::JsonNumber(s.parse().unwrap()));
        }
        if self.parse_char('"') {
            if let Some(s) = self.parse_while(|c| c.is_ascii() && c != '"') {
                if self.parse_char('"') {
                    return Some(JsonValue::JsonString(s));
                }

                return None;
            }

            return None;
        }
        if self.parse_char('[') {
            let mut v: Vec<JsonValue> = Vec::new();
            self.parse_whitespace();
            while !self.parse_char(']') {
                match self.next() {
                    Some(x) => v.push(x),
                    None => return None,
                }
                self.parse_whitespace();
                if self.parse_char(']') {
                    break;
                }
                if !self.parse_char(',') {
                    return None;
                }
                self.parse_whitespace();
            }
            return Some(JsonValue::JsonArray(v));
        }
        if self.parse_char('{') {
            let mut m: HashMap<String, JsonValue> = HashMap::new();
            self.parse_whitespace();
            while !self.parse_char('}') {
                if self.parse_char('"') {
                    if let Some(k) = self.parse_while(|c| c.is_ascii_alphanumeric() && c != '"') {
                        if self.parse_char('"') {
                            self.parse_whitespace();
                            if self.parse_char(':') {
                                self.parse_whitespace();
                                match self.next() {
                                    Some(v) => _ = m.insert(k, v),
                                    None => return None,
                                }
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }

                self.parse_whitespace();
                if self.parse_char('}') {
                    break;
                }
                if !self.parse_char(',') {
                    return None;
                }
                self.parse_whitespace();
            }
            return Some(JsonValue::JsonObject(m));
        }

        return None;
    }
}

fn main() {
    let mut buffer = String::new();
    let mut f = std::io::stdin();
    f.read_to_string(&mut buffer).unwrap();

    let input: Input<char> = Input::new(buffer.chars().collect::<Vec<char>>());

    for item in input {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Input, JsonValue};

    #[test]
    fn test_full() {
        let input: Input<char> = Input::new(
            "{\"key1\": {\"key1\": \"value\", \"key2\": [1, 2, 3]}, \"key2\": [5, 4, 3, 2, {\"key1\": 0}]}"
                .chars()
                .collect::<Vec<char>>(),
        );

        let json = input.into_iter().next().unwrap();

        let json_true = JsonValue::JsonObject(HashMap::from([
            (
                "key1".to_string(),
                JsonValue::JsonObject(HashMap::from([
                    (
                        "key1".to_string(),
                        JsonValue::JsonString("value".to_string()),
                    ),
                    (
                        "key2".to_string(),
                        JsonValue::JsonArray(vec![
                            JsonValue::JsonNumber(1),
                            JsonValue::JsonNumber(2),
                            JsonValue::JsonNumber(3),
                        ]),
                    ),
                ])),
            ),
            (
                "key2".to_string(),
                JsonValue::JsonArray(vec![
                    JsonValue::JsonNumber(5),
                    JsonValue::JsonNumber(4),
                    JsonValue::JsonNumber(3),
                    JsonValue::JsonNumber(2),
                    JsonValue::JsonObject(HashMap::from([(
                        "key1".to_string(),
                        JsonValue::JsonNumber(0),
                    )])),
                ]),
            ),
        ]));

        assert_eq!(json, json_true);
    }
}
