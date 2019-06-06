jp
==

jq but with [JSONPath](https://goessner.net/articles/JsonPath/).


``` shell
$ jp -r '$.some' <<< '{"some": ["value", 3]}'
"value"
3
```

``` shell
$ jp <<< '{"some": ["value", 3]}'
{
  "some": [
    "value",
    3
  ]
}
```

## Rationale

1. JSONPath is more portable than jq's own query language.
   Once you learn the jq language, you now master jq, but this knowledge can probably not be applied any place else. There are many implementations of JSONPath, in all the popular languages, you are very likely to meet again.

2. jq might be doing too many things. Unix probably already has some tooling for that. Let's not reinvent the wheel. jp doesn't plan on becoming too big.
