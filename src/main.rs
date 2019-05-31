use std::io;
use std::env;
extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

fn main() {
    let selector = env::args().nth(1)
        .expect("Need a selector");
    let selector = Selector::new(&selector)
        .expect("Unable to parse selector");

    let json: Value = serde_json::from_reader(io::stdin()).unwrap();

    selector.find(&json)
        .map(|m| m.to_string())
        .for_each(|e| println!("{}", e));
}
