#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Number(f64),
    Error(String),
    String(String),
    Array(Vec<Value>)
}