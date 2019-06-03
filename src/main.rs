use std::io;
extern crate jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App};
use jsonpath::Selector;
use serde_json::Value;

fn do_query(selector: &str, json: Value, show_raw: bool) {
    let selector = Selector::new(selector)
        .expect("Unable to parse selector");

    let matches = selector.find(&json);

    if show_raw {
        matches
            .map(|m| serde_json::to_string(&m).unwrap())
            .for_each(|m| {
                println!("{}", m);
            });
    } else {
        let m: Vec<&Value> = matches.collect();
        serde_json::to_writer(io::stdout(), &m)
            .expect("Unable to serialize JSON");
    }
}

fn pretty_print(json: Value) {
    serde_json::to_writer_pretty(io::stdout(), &json)
        .expect("Unable to format JSON");
}

fn main() {
    let matches = App::new("jp")
        .version("0.0.1")
        .about("jq but with JSONPath")
        .arg(Arg::with_name("r")
             .short("r")
             .requires("SELECTOR")
             .help("Returns one entry per line"))
        .arg(Arg::with_name("SELECTOR")
             .help("JSONPath selector")
             .index(1))
        .get_matches();

    let json: Value = serde_json::from_reader(io::stdin())
        .expect("Unable to parse JSON");

    if matches.is_present("SELECTOR") {
        do_query(matches.value_of("SELECTOR").unwrap(), json, matches.is_present("r"))
    } else {
        pretty_print(json);
    }
}
