use core::str;
use std::io::BufRead;

use crate::value::Value;

#[derive(Debug)]
pub struct Parser<T> {
    buffer: T,
}

impl<T: BufRead> Parser<T> {
    pub fn new(buffer: T) -> Self {
        Self { buffer }
    }

    fn read_to_crlf(self: &mut Self) -> Result<String, std::io::Error> {
        let buffer = match self.buffer.fill_buf() {
            Ok(buffer) => buffer,
            Err(err) => return Err(err),
        };

        let consumed = buffer
            .iter()
            .take_while(|c| **c != b'\n' && **c != b'\r')
            .count();

        let total = consumed
            + if consumed < buffer.len() {
                // we found a delimiter
                if consumed + 1 < buffer.len() // we look if we found two delimiter
                && buffer[consumed] == b'\r'
                && buffer[consumed + 1] == b'\n'
                {
                    2
                } else {
                    1
                }
            } else {
                0
            };

        let text = match str::from_utf8(&buffer[..consumed]) {
            Ok(line) => line.to_string(),
            Err(err) => panic!("{err}"),
        };

        self.buffer.consume(total);
        return Ok(text);
    }

    fn read_string(self: &mut Self, count: usize) -> String {
        let mut buf = vec![0u8; count as usize];
        self.buffer.read_exact(&mut buf).unwrap();
        let _ = self.read_to_crlf();
        str::from_utf8(&buf).unwrap().to_string()
    }

    pub fn parse(self: &mut Self) -> Value {
        let line = match self.read_to_crlf() {
            Ok(text) => text,
            Err(_) => return Value::Error("Parsing error".to_string()),
        };

        let _type = match line.chars().nth(0) {
            Some(t) => t,
            None => {
                return Value::Error("Incorrect command format".to_string())
            },
        };
        let command = line[1..].to_string();

        match _type {
            '+' => Value::String(command),
            '-' => Value::Error(command),
            ':' => Value::Number(command.parse().unwrap()),
            '$' => {
                if command == "-1" {
                    return Value::Null;
                }
                let count = command.parse().unwrap();
                let text = self.read_string(count);
                println!("text {text}");
                Value::String(text)
            }
            '*' => {
                let count = command.parse().unwrap();
                let mut arr: Vec<Value> = Vec::with_capacity(count);
                for _ in 0..count {
                    arr.push(self.parse());
                }
                Value::Array(arr)
            }
            _ => Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    // Parsing RESP line with Parser
    fn parse(line: &str) -> Value {
            let mut parser = Parser::new(BufReader::new(line.as_bytes()));
            parser.parse()
    }

    #[test]
    fn test_null() {
        let value = parse("$-1\r\n");
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_ping_command() {
        let value = parse("*1\r\n$4\r\nping\r\n");
        assert_eq!(value, Value::Array(vec![Value::String("ping".to_string())]));
    }

    #[test]
    fn test_echo_command() {
        let value = parse("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
        assert_eq!(
            value,
            Value::Array(vec![
                Value::String("echo".to_string()),
                Value::String("hello world".to_string()),
            ])
        );
    }

    #[test]
    fn test_get_command() {
        let value = parse("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
        assert_eq!(
            value,
            Value::Array(vec![
                Value::String("get".to_string()),
                Value::String("key".to_string()),
            ])
        );
    }

    #[test]
    fn test_text_ok() {
        let value = parse("+OK\r\n");
        assert_eq!(value, Value::String("OK".to_string()));
    }

    #[test]
    fn test_error_message() {
        let value = parse("-Error message\r\n");
        assert_eq!(value, Value::Error("Error message".to_string()));
    }

    #[test]
    fn test_empty_text() {
        let value = parse("$0\r\n\r\n");
        assert_eq!(value, Value::String("".to_string()));
    }

    #[test]
    fn test_text_hello_world() {
        let value = parse("+hello world\r\n");
        assert_eq!(value, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_number() {
        let value = parse(":123\r\n");
        assert_eq!(value, Value::Number(123.0));
    }

    #[test]
    fn test_nested_array() {
        let value = parse("*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n");
        assert_eq!(
            value,
            Value::Array(vec![
                Value::Array(vec![
                    Value::Number(1.0),
                    Value::Number(2.0),
                    Value::Number(3.0),
                ]),
                Value::Array(vec![
                    Value::String("Foo".to_string()),
                    Value::Error("Bar".to_string()),
                ])
            ])
        );
    }

    #[test]
    fn test_large_text() {
        let value = parse("$11\r\nhello\nworld\r\n");
        assert_eq!(value, Value::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_large_text_with_crlf() {
        let value = parse("$12\r\nhello\r\nworld\r\n");
        assert_eq!(value, Value::String("hello\r\nworld".to_string()));
    }
}
