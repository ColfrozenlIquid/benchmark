use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct JsonParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.pos != self.input.len() {
            return Err("Extra characters after JSON value".into());
        }
        Ok(value)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() { self.pos += 1; } else { break; }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        match self.current_char() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string().map(JsonValue::String),
            Some(c) if c == '-' || c.is_digit(10) => self.parse_number().map(JsonValue::Number),
            Some('t') => self.expect_literal("true", JsonValue::Bool(true)),
            Some('f') => self.expect_literal("false", JsonValue::Bool(false)),
            Some('n') => self.expect_literal("null", JsonValue::Null),
            _ => Err("Unexpected character".into()),
        }
    }

    fn expect_literal(&mut self, literal: &str, value: JsonValue) -> Result<JsonValue, String> {
        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            Ok(value)
        } else {
            Err(format!("Expected literal {}", literal))
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.pos += 1; // consume '{'
        self.skip_whitespace();
        let mut object = HashMap::new();
        if let Some('}') = self.current_char() {
            self.pos += 1;
            return Ok(JsonValue::Object(object));
        }
        loop {
            self.skip_whitespace();
            if self.current_char() != Some('"') {
                return Err("Expected string key".into());
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.current_char() != Some(':') {
                return Err("Expected ':' after key".into());
            }
            self.pos += 1; // consume ':'
            self.skip_whitespace();
            let value = self.parse_value()?;
            object.insert(key, value);
            self.skip_whitespace();
            match self.current_char() {
                Some(',') => { self.pos += 1; },
                Some('}') => { self.pos += 1; break; },
                _ => return Err("Expected ',' or '}' in object".into()),
            }
        }
        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.pos += 1; // consume '['
        self.skip_whitespace();
        let mut array = Vec::new();
        if let Some(']') = self.current_char() {
            self.pos += 1;
            return Ok(JsonValue::Array(array));
        }
        loop {
            self.skip_whitespace();
            let value = self.parse_value()?;
            array.push(value);
            self.skip_whitespace();
            match self.current_char() {
                Some(',') => { self.pos += 1; },
                Some(']') => { self.pos += 1; break; },
                _ => return Err("Expected ',' or ']' in array".into()),
            }
        }
        Ok(JsonValue::Array(array))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.pos += 1; // consume opening '"'
        let mut result = String::new();
        while let Some(c) = self.current_char() {
            self.pos += c.len_utf8();
            if c == '"' { return Ok(result); }
            else if c == '\\' {
                if let Some(esc) = self.current_char() {
                    self.pos += esc.len_utf8();
                    match esc {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\x08'),
                        'f' => result.push('\x0C'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        other => result.push(other),
                    }
                } else { return Err("Unterminated escape sequence in string".into()); }
            } else {
                result.push(c);
            }
        }
        Err("Unterminated string".into())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        if self.current_char() == Some('-') { self.pos += 1; }
        while let Some(c) = self.current_char() {
            if c.is_digit(10) { self.pos += c.len_utf8(); } else { break; }
        }
        if self.current_char() == Some('.') {
            self.pos += 1;
            while let Some(c) = self.current_char() {
                if c.is_digit(10) { self.pos += c.len_utf8(); } else { break; }
            }
        }
        let num_str = &self.input[start..self.pos];
        num_str.parse::<f64>().map_err(|e| e.to_string())
    }
}

pub fn serialize_json(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => {
            let mut out = String::from("\"");
            for c in s.chars() {
                if c == '"' || c == '\\' { out.push('\\'); }
                out.push(c);
            }
            out.push('"');
            out
        },
        JsonValue::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(|v| serialize_json(v)).collect();
            format!("[{}]", elems.join(","))
        },
        JsonValue::Object(obj) => {
            let mut pairs = Vec::new();
            for (k, v) in obj {
                pairs.push(format!("\"{}\":{}", k, serialize_json(v)));
            }
            format!("{{{}}}", pairs.join(","))
        },
    }
}