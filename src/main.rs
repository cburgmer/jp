use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App};
use serde_json::{Deserializer, Value};

fn print_raw(value: &Value) {
    if value.is_string() {
        let s = value.as_str().unwrap();
        println!("{}", s);
    } else {
        println!("{}", serde_json::to_string(&value).unwrap());
    }
}

fn print_json(value: &Value) {
    serde_json::to_writer(io::stdout(), &value)
        .expect("Unable to serialize JSON");
    println!("");
}

fn execute_query<'a>(query: &'a str, json: &'a Value) -> Vec<&'a Value> {
    let mut selector = jsonpath::selector(&json);

    selector(query)
        .expect("Unable to parse selector")
}

fn pretty_print(json: Value) {
    serde_json::to_writer_pretty(io::stdout(), &json)
        .expect("Unable to format JSON");
    println!("");
}

fn main() {
    let matches = App::new("jp")
        .version("0.0.1")
        .about("jq but with JSONPath")
        .arg(Arg::with_name("r")
             .short("r")
             .help("Unwraps primitive JSON values"))
        .arg(Arg::with_name("SELECTOR")
             .help("JSONPath selector")
             .index(1))
        .get_matches();

    let stream = Deserializer::from_reader(io::stdin())
        .into_iter::<Value>()
        .map(|v| v.expect("Unable to parse JSON"));

    for json in stream {
        let results = execute_query(matches.value_of("SELECTOR").unwrap_or("$"), &json);

        if matches.is_present("r") {
            results
                .iter()
                .for_each(|e| print_raw(&e));
        } else if matches.is_present("SELECTOR") {
            results
                .iter()
                .for_each(|e| print_json(&e));
        } else {
            pretty_print(json);
        }
    }
}
