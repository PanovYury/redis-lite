use core::str;
use std::io::{BufRead, BufReader};

use crate::value::Value;

#[derive(Debug)]
pub struct Parser<T> {
    buffer: T,
}

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl<T: BufRead> Iterator for Parser<T> {
    type Item = Result<String, MyError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, total) = {
            let buffer = match self.buffer.fill_buf() {
                Ok(buffer) => buffer,
                Err(e) => return Some(Err(MyError::Io(e))),
            };
            if buffer.is_empty() {
                return None;
            }
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
            let line = match str::from_utf8(&buffer[..consumed]) {
                Ok(line) => line.to_string(),
                Err(e) => return Some(Err(MyError::Utf8(e))),
            };
            (line, total)
        };
        self.buffer.consume(total);

        Some(Ok(line))
    }
}

impl<T: BufRead> Parser<T> {
    pub fn new(buffer: T) -> Self {
        Self { buffer }
    }

    pub fn parse(self: &mut Self) -> Value {

        let line = match self.next().unwrap() {
            Ok(l) => l,
            Err(_) => return Value::Error("Incorrect value".to_string()),
        };
        let _type = match line.chars().nth(0) {
            Some(t) => t,
            None => return Value::Error("Incorrect command format".to_string()),
        };
        let command = line[1..].to_string();
        println!("Parsing command {} {}", _type, command);
        match _type {
            '+' => Value::String(command),
            '-' => Value::Error(command),
            ':' => Value::Number(command.parse().unwrap()),
            '$' => {
                if command == "-1" {
                    return Value::Null;
                }
                let text = self.next().unwrap().unwrap();
                println!("get text {text}");
                Value::String(text)
            }
            '*' => {
                let count = command.parse().unwrap();
                println!("parse array with size {count}");
                let mut arr: Vec<Value> = Vec::with_capacity(count);
                for _ in 0..count {
                    println!("push to array");
                    arr.push(self.parse());
                }
                Value::Array(arr)
            }
            _ => Value::Null,
        }
    }
}

pub fn parse(line: &String) -> Value {
    let mut parser = Parser::new(BufReader::new(line.as_bytes()));
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let line = "$-1\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_ping_command() {
        let line = "*1\r\n$4\r\nping\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Array(vec![Value::String("ping".to_string())]));
    }

    #[test]
    fn test_echo_command() {
        let line = "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
        let value = parse(&line.to_string());
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
        let line = "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n";
        let value = parse(&line.to_string());
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
        let line = "+OK\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::String("OK".to_string()));
    }

    #[test]
    fn test_error_message() {
        let line = "-Error message\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Error("Error message".to_string()));
    }

    #[test]
    fn test_empty_text() {
        let line = "$0\r\n\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::String("".to_string()));
    }

    #[test]
    fn test_text_hello_world() {
        let line = "+hello world\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_number() {
        let line = ":123\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Number(123.0));
    }

    #[test]
    fn test_nested_array() {
        let line = "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Array(vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
            Value::Array(vec![
                Value::String("Foo".to_string()),
                Value::Error("Bar".to_string()),
            ])
        ]));
    }

    #[test]
    fn test_large_text() {
        let line = "$11\r\nhello\nworld";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::String("hello\nworld".to_string()));
    }
}
