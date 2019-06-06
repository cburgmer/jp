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

1. JSONPath is more or less a standard implemented in many languages. jq seems
   to ship with its very own idea of a query language. Let's focus on one query
   language we can reuse in other areas.

2. jq is powerful yet has a steep learning curve. Unix on the other hand might
   already solve some of the problems jq addresses. Let's not reinvent the wheel.

## Road map

In absence of a road map here are a few queries from an actual shell history: [goals/JQ_EXAMPLES.md](goals/JQ_EXAMPLES.md)
