use std::{env, io::BufReader};

use redis_lite::resp::Parser;

fn main() {
    match env::args().nth(1) {
        Some(line) => {
            let mut parser = Parser::new(BufReader::new(line.as_bytes()));
            println!("{:?}", parser.parse());
        }
        None => {}
    }
}
