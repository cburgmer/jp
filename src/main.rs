use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App, ArgMatches};
use serde_json::{Deserializer, Value};

fn print_json(value: &Value) {
    serde_json::to_writer(io::stdout(), &value)
        .expect("Unable to serialize JSON");
    println!("");
}

fn print_raw(value: &Value) {
    if value.is_string() {
        let s = value.as_str().unwrap();
        println!("{}", s);
    } else {
        print_json(value);
    }
}

fn print_pretty(value: &Value) {
    serde_json::to_writer_pretty(io::stdout(), &value)
        .expect("Unable to serialize JSON");
    println!("");
}

fn print(values: Vec<&Value>, matches: &ArgMatches) {
    if matches.is_present("r") {
        values
            .iter()
            .for_each(|e| print_raw(&e));
    } else if matches.is_present("SELECTOR") {
        values
            .iter()
            .for_each(|e| print_json(&e));
    } else {
        values
            .iter()
            .for_each(|e| print_pretty(&e));
    }
}

fn execute_query<'a>(query: &'a str, json: &'a Value) -> Vec<&'a Value> {
    let mut selector = jsonpath::selector(&json);

    selector(query)
        .expect("Unable to parse selector")
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
        print(results, &matches);
    }
}
