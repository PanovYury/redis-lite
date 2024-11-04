use crate::value::Value;

fn parse_number(line: &String) -> Value {
    let number = line[1..].trim();
    match number.parse::<f64>() {
        Ok(n) => Value::Number(n),
        Err(err) => Value::Error(err.to_string() + " " + number)
    }
}

fn split_array(line: &String) -> Vec<String> {
    // let count = line.chars().nth(1).unwrap_or('0').to;
    // let count = count.parse
}

pub fn parse(line: &String) -> Value {
    let _type = match line.chars().nth(0) {
        Some(t) => t,
        None => return Value::Error("Incorrect command format".to_string())
    };
    // let value = 
    match _type {
        '+' => Value::String(line[1..].trim().to_string()),
        '-' => Value::Error(line[1..].trim().to_string()),
        ':' => parse_number(&line),
        '$' => Value::String(line[1..].trim().to_string()),
        '*' => Value::Array(vec![]),
        _ => Value::Null
    }
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
        assert_eq!(value, Value::Array(vec![
            Value::String("ping".to_string()),
            Value::String("hello world".to_string()),
        ]));
    }

    #[test]
    fn test_get_command() {
        let line = "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Array(vec![
            Value::String("get".to_string()),
            Value::String("key".to_string()),
        ]));
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
    fn test_empty_array() {
        let line = "$0\r\n\r\n";
        let value = parse(&line.to_string());
        assert_eq!(value, Value::Array(vec![]));
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
