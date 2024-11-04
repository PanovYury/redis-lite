use std::collections::HashMap;

use redis_lite::value::Value;

// use crate::value;

// fn print_key(k: &String, map: &HashMap::<String, Value>) {
//     match map.get(k).unwrap() {
//         Value::String(value) => println!("Key1 is string: {}", value),
//         Value::Number(value) => println!("Key1 is number: {}", value),
//         Value::Null => println!("Key1 is null"),
//     }
// }

fn main() {
    // let mut map = HashMap::<String, Value>::new();
    // map.insert("key1".to_string(), Value::String("Hello".to_string()));
    // map.insert("key2".to_string(), Value::Number(42));

    // print_key(&"key1".to_string(), &map);
    // print_key(&"key2".to_string(), &map);

    let a = Value::String("Hello".to_string());
    let b = Value::String("Hello".to_string());
    let c = Value::String("World".to_string());

    let arr1 = Value::Array(vec![
        Value::String("Hello".to_string()),
        Value::String("Hello".to_string()),
        Value::String("World".to_string())
    ]);

    let arr2 = Value::Array(vec![
        Value::String("Hello".to_string()),
        Value::String("Hello".to_string()),
        Value::String("World".to_string())
    ]);

    let arr3 = Value::Array(vec![
        Value::String("Hello".to_string()),
        // Value::String("Hello".to_string()),
        Value::String("World".to_string())
    ]);

    println!("a == b {}", a == b);
    println!("a == c {}", a == c);

    println!("arr1 == arr2 {}", arr1 == arr2);
    println!("arr1 == arr3 {}", arr1 == arr3);
}
