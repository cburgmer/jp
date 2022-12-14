use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use std::process;

use clap::{Arg, ArgGroup, ArgAction, Command};
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
    serde_json::to_string(&value) .unwrap()
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
    serde_json::to_string_pretty(&value) .unwrap()
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

fn config() -> (Serialization, Formatting, bool, Vec<String>) {
    let mut matches = Command::new("jp")
        .version("0.4.0")
        .about("A simpler jq, and with JSONPath")
        .arg(Arg::new("r")
             .short('r')
             .action(ArgAction::SetTrue)
             .help("Unwraps primitive JSON values"))
        .arg(Arg::new("tabs")
             .short('t')
             .requires("SELECTOR")
             .action(ArgAction::SetTrue)
             .help("Transposes all matches per document, separated by tabs"))
        .arg(Arg::new("print0")
             .short('0')
             .action(ArgAction::SetTrue)
             .help("Separates all matches by NUL (\\0), helpful in conjunction with xargs -0"))
        .arg(Arg::new("example")
             .long("example")
             .action(ArgAction::SetTrue)
             .help("Prints example JSON for practising JSONPath"))
        .arg(Arg::new("SELECTOR")
             .help("JSONPath selector")
             .index(1)
             .num_args(..))
        .group(ArgGroup::new("formatting")
               .args(["tabs", "print0"])
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
    if matches.get_flag("r") {
        serialization = Serialization::Raw;
    } else if matches.contains_id("SELECTOR") {
        serialization = Serialization::JsonBlob;
    } else {
        serialization = Serialization::JsonPretty;
    }

    let formatting: Formatting;
    if matches.get_flag("tabs") {
        formatting = Formatting::Tabs;
    } else if matches.get_flag("print0") {
        formatting = Formatting::Nul
    } else {
        formatting = Formatting::Newlines
    }

    let root_selector = String::from("$");
    let selectors = matches.remove_many::<String>("SELECTOR").map_or(vec![root_selector], |s| s.collect());

    (serialization, formatting, matches.get_flag("example"), selectors)
}

fn main() {
    let (serialization, formatting, output_example, selectors) = config();

    let stream;
    if output_example {
        stream = vec![example()];
    } else {
        stream = Deserializer::from_reader(io::stdin())
            .into_iter::<Value>()
            .map(|v|
                 v.unwrap_or_else(|err| {
                     eprintln!("Unable to parse JSON, {}", err);
                     process::exit(4);
                 })
            )
            .collect::<Vec<_>>();
    }

    let compiled_selectors = selectors.iter().map(|s| jsonpath::Compiled::compile(&s).unwrap_or_else(|err| {
        eprintln!("Unable to parse selector\n{}", err);
        process::exit(3);
    })).collect::<Vec<_>>();

    for json in stream {
        for compiled_selector in &compiled_selectors {
            let results = compiled_selector.select(&json).unwrap();
            let entries = serialize(results, &serialization);

            match formatting {
                Formatting::Tabs => println!("{}", entries.join("\t")),
                Formatting::Nul => entries.iter().for_each(|s| print!("{}\0", s)),
                Formatting::Newlines => entries.iter().for_each(|s| println!("{}", s))
            }
        }
    }
}
