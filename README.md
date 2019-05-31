jp
==

jq but with [JSONPath](https://goessner.net/articles/JsonPath/).


``` shell
$ jp '$.some[0]' <<< '{"some": ["value", 3]}'
"value"
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
