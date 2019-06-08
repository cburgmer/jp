# jp

jp is a JSON processor for the command line using
[JSONPath](https://goessner.net/articles/JsonPath/)
(aka "jq but with JSONPath").


    $ jp -r '$.some' <<< '{"some": ["value", 3]}'
    "value"
    3


    $ jp <<< '{"some": ["value", 3]}'
    {
      "some": [
        "value",
        3
      ]
    }


## Rationale

jq is already quite successful, yet jp wants to improve on that:

1. JSONPath is more or less a standard implemented in many languages. jq seems
   to ship with its very own idea of a query language. Let's focus on one query
   language we can reuse in other areas.

2. jq is powerful yet has a steep learning curve. Unix on the other hand might
   already solve some of the problems jq addresses. Let's not reinvent the wheel.


## Goals

In absence of a roadmap here are a few queries from an actual shell history:
[goals/JQ_EXAMPLES.md](goals/JQ_EXAMPLES.md)

Also so far unanswered questions: [goals/Questions.md](goals/Questions.md)
