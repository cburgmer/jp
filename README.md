jp
==

jq but with [JSONPath](https://goessner.net/articles/JsonPath/).


``` shell
$ jp -r '$.some.*' <<< '{"some": ["value", 3]}'
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
