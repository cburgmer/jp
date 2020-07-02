use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, ArgGroup, App};
use serde_json::{Deserializer, Value};

use jp::example;

enum Serialization {
    Raw,
    JsonBlob,
    JsonPretty
}

enum Formatting {
    Tabs,
    Nul,
    Newlines
}

fn serialize_json(value: &Value) -> String {
    serde_json::to_string(&value)
        .expect("Unable to serialize JSON")
}

fn serialize_raw(value: &Value) -> String {
    if value.is_string() {
        value.as_str().unwrap().to_string()
    } else  if value.is_null() {
        "".to_string()
    } else {
        serialize_json(value)
    }
}

fn serialize_pretty(value: &Value) -> String {
    serde_json::to_string_pretty(&value)
        .expect("Unable to serialize JSON")
}

fn serialize(values: Vec<&Value>, serialization: &Serialization) -> Vec<String> {
    values
        .iter()
        .map(|v| {
            match serialization {
                Serialization::Raw => serialize_raw(&v),
                Serialization::JsonBlob => serialize_json(&v),
                Serialization::JsonPretty => serialize_pretty(&v)
            }
        })
        .collect()
}

fn config() -> (Serialization, Formatting, bool, String) {
    let matches = App::new("jp")
        .version("0.4.0")
        .about("A simpler jq, and with JSONPath")
        .arg(Arg::with_name("r")
             .short("r")
             .help("Unwraps primitive JSON values"))
        .arg(Arg::with_name("tabs")
             .short("t")
             .requires("SELECTOR")
             .help("Transposes all matches per document, separated by tabs"))
        .arg(Arg::with_name("print0")
             .short("0")
             .help("Separates all matches by NUL (\\0), helpful in conjunction with xargs -0"))
        .arg(Arg::with_name("example")
             .long("example")
             .help("Prints example JSON for practising JSONPath"))
        .arg(Arg::with_name("SELECTOR")
             .help("JSONPath selector")
             .index(1))
        .group(ArgGroup::with_name("formatting")
               .args(&["tabs", "print0"])
               .multiple(false))
        .after_help("SELECTOR EXAMPLES:
    array index\t\t$[2]
    object key\t\t$.key
    complex object key\t$['a key']
    union\t\t$['key','another']
    array slice\t\t$[0:4]
    filter expression\t$[?(@.key==42)]
    recursive descent\t$..key
    wildcard\t\t$.*

E.g. get the prices of everything in the store:
  jp --example | jp '$.store..price'
")
        .get_matches();

    let serialization: Serialization;
    if matches.is_present("r") {
        serialization = Serialization::Raw;
    } else if matches.is_present("SELECTOR") {
        serialization = Serialization::JsonBlob;
    } else {
        serialization = Serialization::JsonPretty;
    }

    let formatting: Formatting;
    if matches.is_present("tabs") {
        formatting = Formatting::Tabs;
    } else if matches.is_present("print0") {
        formatting = Formatting::Nul
    } else {
        formatting = Formatting::Newlines
    }

    let selector = matches.value_of("SELECTOR").unwrap_or("$");

    (serialization, formatting, matches.is_present("example"), selector.to_string())
}

fn main() {
    let (serialization, formatting, output_example, selector) = config();

    let mut select = jsonpath::compile(&selector);

    let stream;
    if output_example {
        stream = vec![example()];
    } else {
        stream = Deserializer::from_reader(io::stdin())
            .into_iter::<Value>()
            .map(|v| v.expect("Unable to parse JSON"))
            .collect::<Vec<_>>();
    }

    for json in stream {
        let results = select(&json).expect("Unable to parse selector");
        let entries = serialize(results, &serialization);

        match formatting {
            Formatting::Tabs => println!("{}", entries.join("\t")),
            Formatting::Nul => entries.iter().for_each(|s| print!("{}\0", s)),
            Formatting::Newlines => entries.iter().for_each(|s| println!("{}", s))
        }
    }
}
