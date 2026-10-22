//! Minimal dependency-free JSON reader/writer.
//!
//! The benchmark definitions under `benchmark/` are JSONL files, and the run
//! reports are JSON. The workspace deliberately has no third-party crates, so
//! this module implements exactly the subset of RFC 8259 that the benchmark
//! needs: objects, arrays, strings, numbers, booleans and null.
//!
//! Numbers keep their lexical shape so that `30` and `30.0` stay distinguishable
//! while parsing, which lets the case loader tell integer literals from float
//! literals without a schema.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use metagdb_core::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    pub fn as_object(&self) -> Option<&BTreeMap<String, Json>> {
        match self {
            Json::Object(entries) => Some(entries),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Json]> {
        match self {
            Json::Array(items) => Some(items),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Text(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Field lookup on an object; `None` for a non-object or a missing key.
    pub fn get(&self, key: &str) -> Option<&Json> {
        self.as_object().and_then(|entries| entries.get(key))
    }

    /// Write the value as compact single-line JSON.
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(true) => out.push_str("true"),
            Json::Bool(false) => out.push_str("false"),
            Json::Int(value) => {
                let _ = write!(out, "{value}");
            }
            Json::Float(value) => {
                if value.is_finite() {
                    let _ = write!(out, "{value}");
                } else {
                    out.push_str("null");
                }
            }
            Json::Text(value) => write_escaped(out, value),
            Json::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            Json::Object(entries) => {
                out.push('{');
                for (index, (key, value)) in entries.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_escaped(out, key);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
        }
    }
}

fn write_escaped(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Parse one complete JSON document. Trailing non-whitespace input is an error.
pub fn parse(text: &str) -> Result<Json> {
    let mut parser = Parser {
        bytes: text.as_bytes(),
        position: 0,
    };
    parser.skip_whitespace();
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    if parser.position != parser.bytes.len() {
        return Err(parser.error("trailing content after JSON value"));
    }
    Ok(value)
}

struct Parser<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    fn error(&self, message: &str) -> Error {
        Error::Other(format!("json: {message} at byte {}", self.position))
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(
            self.peek(),
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
        ) {
            self.position += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Result<()> {
        if self.peek() == Some(byte) {
            self.position += 1;
            Ok(())
        } else {
            Err(self.error(&format!("expected `{}`", byte as char)))
        }
    }

    fn parse_value(&mut self) -> Result<Json> {
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(Json::Text(self.parse_string()?)),
            Some(b't') => self.parse_literal("true", Json::Bool(true)),
            Some(b'f') => self.parse_literal("false", Json::Bool(false)),
            Some(b'n') => self.parse_literal("null", Json::Null),
            Some(_) => self.parse_number(),
            None => Err(self.error("unexpected end of input")),
        }
    }

    fn parse_literal(&mut self, word: &str, value: Json) -> Result<Json> {
        if self.bytes[self.position..].starts_with(word.as_bytes()) {
            self.position += word.len();
            Ok(value)
        } else {
            Err(self.error("invalid literal"))
        }
    }

    fn parse_object(&mut self) -> Result<Json> {
        self.expect(b'{')?;
        let mut entries = BTreeMap::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.position += 1;
            return Ok(Json::Object(entries));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            self.skip_whitespace();
            let value = self.parse_value()?;
            entries.insert(key, value);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.position += 1;
                }
                Some(b'}') => {
                    self.position += 1;
                    return Ok(Json::Object(entries));
                }
                _ => return Err(self.error("expected `,` or `}` in object")),
            }
        }
    }

    fn parse_array(&mut self) -> Result<Json> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b']') {
            self.position += 1;
            return Ok(Json::Array(items));
        }
        loop {
            self.skip_whitespace();
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.position += 1;
                }
                Some(b']') => {
                    self.position += 1;
                    return Ok(Json::Array(items));
                }
                _ => return Err(self.error("expected `,` or `]` in array")),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let byte = self
                .peek()
                .ok_or_else(|| self.error("unterminated string"))?;
            match byte {
                b'"' => {
                    self.position += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.position += 1;
                    let escape = self
                        .peek()
                        .ok_or_else(|| self.error("unterminated escape"))?;
                    self.position += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let code = self.parse_hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&code) {
                                // High surrogate: requires a following low surrogate.
                                if self.peek() != Some(b'\\') {
                                    return Err(self.error("lone high surrogate"));
                                }
                                self.position += 1;
                                if self.peek() != Some(b'u') {
                                    return Err(self.error("lone high surrogate"));
                                }
                                self.position += 1;
                                let low = self.parse_hex4()?;
                                if !(0xDC00..0xE000).contains(&low) {
                                    return Err(self.error("invalid low surrogate"));
                                }
                                let combined = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                                char::from_u32(combined)
                                    .ok_or_else(|| self.error("invalid surrogate pair"))?
                            } else {
                                char::from_u32(code)
                                    .ok_or_else(|| self.error("invalid unicode escape"))?
                            };
                            out.push(ch);
                        }
                        _ => return Err(self.error("unknown escape sequence")),
                    }
                }
                _ => {
                    // Copy one whole UTF-8 scalar.
                    let start = self.position;
                    let width = utf8_width(byte);
                    if start + width > self.bytes.len() {
                        return Err(self.error("truncated UTF-8 sequence"));
                    }
                    let slice = &self.bytes[start..start + width];
                    let text = std::str::from_utf8(slice)
                        .map_err(|_| self.error("invalid UTF-8 in string"))?;
                    out.push_str(text);
                    self.position += width;
                }
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u32> {
        if self.position + 4 > self.bytes.len() {
            return Err(self.error("truncated unicode escape"));
        }
        let text = std::str::from_utf8(&self.bytes[self.position..self.position + 4])
            .map_err(|_| self.error("invalid unicode escape"))?;
        let value =
            u32::from_str_radix(text, 16).map_err(|_| self.error("invalid unicode escape"))?;
        self.position += 4;
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<Json> {
        let start = self.position;
        if self.peek() == Some(b'-') || self.peek() == Some(b'+') {
            self.position += 1;
        }
        let mut is_float = false;
        while let Some(byte) = self.peek() {
            match byte {
                b'0'..=b'9' => self.position += 1,
                b'.' | b'e' | b'E' => {
                    is_float = true;
                    self.position += 1;
                }
                b'+' | b'-' => self.position += 1,
                _ => break,
            }
        }
        if start == self.position {
            return Err(self.error("expected a value"));
        }
        let text = std::str::from_utf8(&self.bytes[start..self.position])
            .map_err(|_| self.error("invalid number"))?;
        if is_float {
            let value: f64 = text.parse().map_err(|_| self.error("invalid float"))?;
            Ok(Json::Float(value))
        } else {
            match text.parse::<i64>() {
                Ok(value) => Ok(Json::Int(value)),
                Err(_) => {
                    let value: f64 = text.parse().map_err(|_| self.error("invalid number"))?;
                    Ok(Json::Float(value))
                }
            }
        }
    }
}

fn utf8_width(first: u8) -> usize {
    if first < 0x80 {
        1
    } else if first >> 5 == 0b110 {
        2
    } else if first >> 4 == 0b1110 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_objects_arrays_and_numbers() {
        let value = parse(r#"{"a":[1,2.5,-3],"b":{"c":true},"d":null}"#).unwrap();
        assert_eq!(value.get("a").unwrap().as_array().unwrap().len(), 3);
        assert_eq!(value.get("a").unwrap().as_array().unwrap()[0], Json::Int(1));
        assert_eq!(
            value.get("a").unwrap().as_array().unwrap()[1],
            Json::Float(2.5)
        );
        assert_eq!(
            value.get("a").unwrap().as_array().unwrap()[2],
            Json::Int(-3)
        );
        assert_eq!(value.get("b").unwrap().get("c").unwrap(), &Json::Bool(true));
        assert_eq!(value.get("d").unwrap(), &Json::Null);
    }

    #[test]
    fn distinguishes_integer_and_float_lexemes() {
        assert_eq!(parse("30").unwrap(), Json::Int(30));
        assert_eq!(parse("30.0").unwrap(), Json::Float(30.0));
        assert_eq!(parse("1e3").unwrap(), Json::Float(1000.0));
    }

    #[test]
    fn round_trips_strings_with_escapes() {
        let value = parse(r#""a\"b\\c\né😀""#).unwrap();
        assert_eq!(value.as_str().unwrap(), "a\"b\\c\né😀");
        let encoded = value.to_string();
        assert_eq!(parse(&encoded).unwrap(), value);
    }

    #[test]
    fn rejects_trailing_content() {
        assert!(parse("{} garbage").is_err());
        assert!(parse("").is_err());
    }
}
