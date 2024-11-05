use std::collections::HashMap;

use redis_lite::{resp::parse, value::Value};

fn main() {
    let line = "$-1\r\n";
    let value = parse(&line.to_string());
    println!("{:?}", value);
}
