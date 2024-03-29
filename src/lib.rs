use std::io;
use std::io::Write;
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate clap;

use std::process;

use clap::{Arg, ArgGroup, ArgAction, Command};
use serde_json::{Deserializer, Value};

fn example() -> Value {
    let example = r#"{
    "store": {
        "book": [
            {
                "category": "reference",
                "author": "Nigel Rees",
                "title": "Sayings of the Century",
                "price": 8.95
            },
            {
                "category": "fiction",
                "author": "Evelyn Waugh",
                "title": "Sword of Honour",
                "price": 12.99
            },
            {
                "category": "fiction",
                "author": "Herman Melville",
                "title": "Moby Dick",
                "isbn": "0-553-21311-3",
                "price": 8.99
            },
            {
                "category": "fiction",
                "author": "J. R. R. Tolkien",
                "title": "The Lord of the Rings",
                "isbn": "0-395-19395-8",
                "price": 22.99
            }
        ],
        "bicycle": {
            "color": "red",
            "price": 19.95
        }
    }
}"#;

    serde_json::from_str(example).unwrap()
}

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

pub struct Config {
    serialization: Serialization,
    formatting: Formatting,
    use_example: bool,
    selectors: Vec<String>
}

impl Config {
    pub fn new() -> Config {
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
                 .help("JSONPath selector(s)")
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
        let use_example = matches.get_flag("example");

        Config{serialization, formatting, use_example, selectors}
    }
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

fn format(entries: Vec<String>, formatting: &Formatting) -> String{
    match formatting {
        Formatting::Tabs => format!("{}\n", entries.join("\t")),
        Formatting::Nul => entries.iter().map(|s| format!("{}\0", s)).collect::<Vec<_>>().join(""),
        Formatting::Newlines => entries.iter().map(|s| format!("{}\n", s)).collect::<Vec<_>>().join("")
    }
}

pub fn run(config: Config) -> std::io::Result<()> {
    let stream;
    if config.use_example {
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

    let compiled_selectors = config.selectors.into_iter().map(|s| jsonpath::Compiled::compile(&s).unwrap_or_else(|err| {
        eprintln!("Unable to parse selector\n{}", err);
        process::exit(3);
    })).collect::<Vec<_>>();

    for json in stream {
        let mut results = vec![];
        for compiled_selector in &compiled_selectors {
            results.append(&mut compiled_selector.select(&json).unwrap());
        }

        let output = format(serialize(results, &config.serialization), &config.formatting);
        // Avoid failing if pipe closed https://github.com/rust-lang/rust/issues/46016
        write!(std::io::stdout(), "{}", output)?
    }
    Ok(())
}
