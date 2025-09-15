mod server;
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut result = HashMap::new();
    let count = 0;

    for word in text.split_whitespace() {
        result.entry(word).or_insert(count + 1);
    }
    println!("{:?}", result);
}
