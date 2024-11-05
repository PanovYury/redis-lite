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

    pub fn parse(self: &mut Self, line: &String) -> Value {
        let mut parser = Parser::new(BufReader::new(line.as_bytes()));
        let mut result = Value::Null;

        while let Some(line) = self.next() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let _type = match line.chars().nth(0) {
                Some(t) => t,
                None => return Value::Error("Incorrect command format".to_string()),
            };
            let command = line[1..].to_string();
            println!("Parsing command {} {}", _type, command);
            result = match _type {
                '+' => Value::String(command),
                '-' => Value::Error(command),
                ':' => Value::Number(command.parse().unwrap()),
                '$' => {
                    if command == "-1" {
                        return Value::Null;
                    }
                    let text = parser.next().unwrap().unwrap();
                    Value::String(text)
                }
                '*' => {
                    let count = command.parse().unwrap();
                    let arr: Vec<Value> = Vec::with_capacity(count);
                    Value::Array(arr)
                }
                _ => Value::Null,
            };
        }
        result
    }
}

// fn parse_number(line: &String) -> Value {
//     let number = line[1..].trim();
//     match number.parse::<f64>() {
//         Ok(n) => Value::Number(n),
//         Err(err) => Value::Error(err.to_string() + " " + number)
//     }
// }

// fn split_array(line: &String) -> Vec<String> {
//     // let count = line.chars().nth(1).unwrap_or('0').to;
//     // let count = count.parse
// }

pub fn parse(line: &String) -> Value {
    let mut parser = Parser::new(BufReader::new(line.as_bytes()));
    let mut result = Value::Null;

    while let Some(line) = parser.next() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let _type = match line.chars().nth(0) {
            Some(t) => t,
            None => return Value::Error("Incorrect command format".to_string()),
        };
        let command = line[1..].to_string();
        println!("Parsing command {} {}", _type, command);
        result = match _type {
            '+' => Value::String(command),
            '-' => Value::Error(command),
            ':' => Value::Number(command.parse().unwrap()),
            '$' => {
                if command == "-1" {
                    return Value::Null;
                }
                let text = parser.next().unwrap().unwrap();
                Value::String(text)
            }
            '*' => {
                let count = command.parse().unwrap();
                let arr: Vec<Value> = Vec::with_capacity(count);
                Value::Array(arr)
            }
            _ => Value::Null,
        };
    }
    result
    // for line in parser {

    // }
    // let commands = line.split("\r\n");
    // if line == "$-1\r\n" || line == "*-1\r\n" {}
    // let _type = match line.chars().nth(0) {
    //     Some(t) => t,
    //     None => return Value::Error("Incorrect command format".to_string())
    // };
    // // let value =
    // match _type {
    //     '+' => Value::String(line[1..].trim().to_string()),
    //     '-' => Value::Error(line[1..].trim().to_string()),
    //     ':' => parse_number(&line),
    //     '$' => Value::String(line[1..].trim().to_string()),
    //     '*' => Value::Array(vec![]),
    //     _ => Value::Null
    // }
    // return Value::Null;
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
                Value::String("ping".to_string()),
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
}
