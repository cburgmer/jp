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
        .try_for_each(|m| serde_json::to_writer(io::stdout(), &m))
        .expect("Unable to serialize JSON");
}

fn pretty_print(json: Value) {
    serde_json::to_writer_pretty(io::stdout(), &json)
        .expect("Unable to format JSON");
}

fn main() {
    let json: Value = serde_json::from_reader(io::stdin())
        .expect("Unable to parse JSON");

    match env::args().nth(1) {
        Some(selector) => do_query(selector, json),
        None => pretty_print(json),
    }
}
