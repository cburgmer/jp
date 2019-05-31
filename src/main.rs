use std::io;
use std::env;
extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

fn do_query(selector: String, json: Value) {
    let selector = Selector::new(&selector)
        .expect("Unable to parse selector");

    selector.find(&json)
        .map(|m| m.to_string())
        .for_each(|e| println!("{}", e));
}

fn pretty_print(json: Value) {
    let pretty_json = serde_json::to_string_pretty(&json)
        .expect("Unable to format JSON");
    println!("{}", pretty_json);
}

fn main() {
    let json: Value = serde_json::from_reader(io::stdin())
        .expect("Unable to parse JSON");

    match env::args().nth(1) {
        Some(selector) => do_query(selector, json),
        None => pretty_print(json),
    }
}
