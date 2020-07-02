Howto jp
========

jp tries to be as straightforward as possible. Part of this goal is having a
clear set of steps a call to jp goes through:

1. Parsing of JSON input, followed by
2. selection (optional), then
3. serialization of results, and finally
4. formatting.


(1) Parsing of JSON input
-------------------------

No surprises here. jp will parse a set of JSON documents from STDIN.
We will probably never support another format.

(2) Selection
-------------

By providing a JSONPath selector, we can narrow down the input documents.
Following the JSONPath algorithm a set of matches will be returned.
Selection is optional - if no selector is provided the input documents are
passed on.

(3) Serialization of results
----------------------------

Currently the following modes of serialization are supported:

1. JSON blobs (default)
2. Pretty JSON
3. Raw JSON values

By default the matches are just serialized as JSON in the most compact form.
Optionally they can be output in a prettier, human readable form with
indentation and line breaks. All we need to do is omit any serialization option.

    $ echo '["a string", 3]' | jp
    [
      "a string",
      3
    ]

If we plan on moving away from JSON, we can select raw JSON values, which will
try to unwrap raw values (thus losing quotes for strings).

    $ echo '["a string", 3]' | jp -r '$.*'
    a string
    3

(4) Formatting
--------------

We can format output in different ways to support further processing of our
results. This allows us to seamlessly move from jp to the Unix world:

1. Newlines (default)
2. Tabs separated
3. NUL (\0) separated

By default matches are printed one per line. (For multiple input documents this
will blur the boundaries across documents).

    $ echo '{"a": 1, "b": 2} {"a": "nice", "b": "awesome"}' | jp '$["a","b"]'
    1
    2
    "nice"
    "awesome"

Tab separation enables the traditional CSV (more precisely TSV) mode to show
tabular data. One line per document.

    $ echo '{"a": 1, "b": 2} {"a": "nice", "b": "awesome"}' | jp -t '$["a","b"]'
    1	2
    "nice"	"awesome"

jp wants to play nicely with xargs (and similar Unix tools), which separation by
NUL char (\0) enables.

    $ echo '{"a": 1, "b": 2} {"a": "nice", "b": "awesome"}' \
      | jp -0 '$["a","b"]' | xargs -0 -n1 -I% echo '{"value": %}'
    {"value": 1}
    {"value": 2}
    {"value": "nice"}
    {"value": "awesome"}

Some pitfalls
-------------

### Matching array entries

Compare `{"key": ["2", 3]}` for `$.key` and `$.key.*`.
The first will return one result `["2", 3]`. The other will return two results:
`"2"` and `3`.
