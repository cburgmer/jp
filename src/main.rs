use std::io;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App};
use serde_json::{Deserializer, Value};

use jp::example;

enum Display {
    Raw,
    OneLine,
    Pretty
}

fn serialize_json(value: &Value) -> String {
    serde_json::to_string(&value)
        .expect("Unable to serialize JSON")
}

fn serialize_raw(value: &Value) -> String {
    if value.is_string() {
        value.as_str().unwrap().to_string()
    } else {
        serialize_json(value)
    }
}

fn serialize_pretty(value: &Value) -> String {
    serde_json::to_string_pretty(&value)
        .expect("Unable to serialize JSON")
}

fn serialize(values: Vec<&Value>, display: &Display) -> Vec<String> {
    values
        .iter()
        .map(|v| {
            match display {
                Display::Raw => serialize_raw(&v),
                Display::OneLine => serialize_json(&v),
                Display::Pretty => serialize_pretty(&v)
            }
        })
        .collect()
}

fn config() -> (Display, bool, bool, String) {
    let matches = App::new("jp")
        .version("0.0.1")
        .about("A simpler jq, and with JSONPath")
        .arg(Arg::with_name("r")
             .short("r")
             .help("Unwraps primitive JSON values"))
        .arg(Arg::with_name("tabs")
             .short("t")
             .requires("SELECTOR")
             .help("Transposes a list of matches separated by tabs"))
        .arg(Arg::with_name("example")
             .long("example")
             .help("Prints example JSON for practising JSONPath"))
        .arg(Arg::with_name("SELECTOR")
             .help("JSONPath selector")
             .index(1))
        .after_help("Supported syntax elements are
$\t\t\tThe root object/element
@\t\t\tThe current object/element
. or []\t\t\tChild operator
..\t\t\tRecursive descent
*\t\t\tWildcard
[]\t\t\tSubscript operator
[,]\t\t\tUnion operator
[start:end:step]\tArray slice operator
?()\t\t\tApplies a filter expression

E.g. get the prices of everything in the store:
  jp --example | jp '$.store..price'
")
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

    (display, matches.is_present("tabs"), matches.is_present("example"), selector.to_string())
}

fn main() {
    let (display, serialize_to_tabs, output_example, selector) = config();

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
        let output = serialize(results, &display);

        if serialize_to_tabs {
            println!("{}", output.join("\t"));
        } else {
            output
                .iter()
                .for_each(|s| println!("{}", s));
        }
    }
}
