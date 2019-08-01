use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App};
use serde_json::{Deserializer, Value};

enum Display {
    Raw,
    OneLine,
    Pretty
}

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

fn print(values: Vec<&Value>, display: &Display) {
    for v in values {
        match display {
            Display::Raw => print_raw(&v),
            Display::OneLine => print_json(&v),
            Display::Pretty => print_pretty(&v)
        }
    }
}

fn config() -> (Display, bool, String) {
    let matches = App::new("jp")
        .version("0.0.1")
        .about("jq but with JSONPath")
        .arg(Arg::with_name("r")
             .short("r")
             .help("Unwraps primitive JSON values"))
        .arg(Arg::with_name("tabs")
             .short("t")
             .help("Transposes a list of matches separated by tabs"))
        .arg(Arg::with_name("SELECTOR")
             .help("JSONPath selector")
             .index(1))
        .get_matches();

    let display: Display;
    if matches.is_present("r") {
        display = Display::Raw;
    } else if matches.is_present("SELECTOR") {
        display = Display::OneLine;
    } else {
        display = Display::Pretty;
    }

    let selector = matches.value_of("SELECTOR").unwrap_or("$");

    (display, matches.is_present("tabs"), selector.to_string())
}

fn main() {
    let (display, serialize_to_tabs, selector) = config();

    let mut select = jsonpath::compile(&selector);

    let stream = Deserializer::from_reader(io::stdin())
        .into_iter::<Value>()
        .map(|v| v.expect("Unable to parse JSON"));

    for json in stream {
        let results = select(&json).expect("Unable to parse selector");
        if serialize_to_tabs {
            let line: String = results.into_iter()
                .map(|v| serde_json::to_string(&v)
                     .expect("Unable to serialize JSON"))
                .collect::<Vec<_>>()
                .join("\t");
            println!("{}", line);
        } else {
            print(results, &display);
        }
    }
}
