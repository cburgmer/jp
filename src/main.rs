use std::io;
use std::process;
use std::env;
extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

fn selector() -> String {
    let selector = env::args().nth(1);
    if selector == None {
        println!("Need a selector");
        process::exit(1);
    }
    selector.unwrap()
}

fn parse_selector(selector : String) -> Selector {
    let selector = Selector::new(&selector);

    if selector.is_err() {
        println!("Unable to parse selector");
        process::exit(1);
    }

    selector.unwrap()
}

fn main() {
    let selector = parse_selector(selector());

    let json: Value = serde_json::from_reader(io::stdin()).unwrap();

    selector.find(&json)
        .map(|m| m.to_string())
        .for_each(|e| println!("{}", e));
}
