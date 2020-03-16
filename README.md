# jp

jp is a JSON processor for the command line using
[JSONPath](https://goessner.net/articles/JsonPath/)
(aka "a simpler jq, and with JSONPath").


    $ jp -r '$.some.*' <<< '{"some": ["value", 3]}'
    value
    3


    $ jp <<< '{"some": ["value", 3]}'
    {
      "some": [
        "value",
        3
      ]
    }


jp uses [jsonpath_lib](https://github.com/freestrings/jsonpath) under the hood.
You can check its [implementation of JSONPath here](https://cburgmer.github.io/json-path-comparison/).

[![Build Status](https://travis-ci.org/cburgmer/jp.svg?branch=master)](https://travis-ci.org/cburgmer/jp)

## Features

    $ jp --help
    jp 0.0.1
    A simpler jq, and with JSONPath

    USAGE:
        jp [FLAGS] [SELECTOR]

    FLAGS:
            --example    Prints example JSON for practising JSONPath
        -h, --help       Prints help information
        -r               Unwraps primitive JSON values
        -t               Transposes a list of matches separated by tabs
        -V, --version    Prints version information

    ARGS:
        <SELECTOR>    JSONPath selector

    SELECTOR EXAMPLES:
        array index         $[2]
        object key          $.key
        complex object key  $['a key']
        union               $['key','another']
        array slice         $[0:4]
        filter expression   $[?(@.key==42)]
        recursive descent   $..key
        wildcard            $.*

    E.g. get the prices of everything in the store:
      jp --example | jp '$.store..price'

## Rationale

jq is quite successful, but has a steep learning curve. jp wants to be simpler:

1. JSONPath is a standard (more or less) implemented in many languages (compare
   https://cburgmer.github.io/json-path-comparison/). jq ships with its very
   own idea of a query language. Let's focus on one query language we can reuse
   in other areas.

2. jq is powerful and complex. Unix on the other hand already solves some of
   the problems jq addresses. Let's not reinvent the wheel.


## Goals

In absence of a roadmap here are a few queries from an actual shell history:
[goals/JQ_EXAMPLES.md](goals/JQ_EXAMPLES.md)

Also so far unanswered questions: [goals/Questions.md](goals/Questions.md)
