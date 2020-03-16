#!/usr/bin/env bats

some_json_input() {
    echo '{"1": ["2", 3]}'
}

some_json_stream() {
    echo '{"a": ["b", 0]}'
    echo '{"a": ["ccc", 1]}'
}

primitives_json_stream() {
    echo '"str"'
    echo 42
}


@test "prints help" {
    jp -h | grep 'A simpler jq, and with JSONPath'
}

@test "prints a version number" {
    jp --version | grep 'jp [0-9]\.[0-9]\.[0-9]'
}

@test "fails on an invalid option" {
    run jp -b

    [ "$status" -eq 1 ]
}

@test "fails on invalid JSON" {
    {
        run jp
    } <<< $(echo 'INVALID')

    [ "$status" -ne 0 ]
    [[ "$output" =~ "Unable to parse JSON" ]]
}

@test "formats JSON input" {
    result="$(some_json_input | jp)"
    expected_output='{
  "1": [
    "2",
    3
  ]
}'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "executes a JSONPath selector" {
    result="$(some_json_input | jp '$.1')"

    [ "$result" = '["2",3]' ]
}

@test "lists multiple results on one line each" {
    result="$(some_json_input | jp '$.1.*')"

    expected_output='"2"
3'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "fails on an invalid JSONPath selector" {
    {
        run jp INVALID
    } <<< $(echo '{}')

    [ "$status" -ne 0 ]
    [[ "$output" =~ "Unable to parse selector" ]]
}

@test "unwraps a number for -r" {
    result="$(echo '{"key": 42}' | jp -r '$.key')"
    expected_output='42'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "unwraps a string for -r" {
    result="$(echo '"a string"' | jp -r '$')"
    expected_output='a string'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "unwraps a list of strings for -r" {
    result="$(echo '["a string", "another"]' | jp -r '$.*')"
    expected_output='a string
another'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "does not unwrap a scalar array match for -r" {
    result="$(echo '["a string", "another"]' | jp -r '$')"
    expected_output='["a string","another"]'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "serialized a complex JSON object for -r" {
    result="$(echo '{"key": 42}' | jp -r '$')"
    expected_output='{"key":42}'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "unwraps all matches for -r" {
    result="$(some_json_input | jp -r '$.1.*')"
    expected_output='2
3'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "returns a scalar array match on one line" {
    result="$(some_json_input | jp -r '$.1')"
    expected_output='["2",3]'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "returns the matches with -r on a single result" {
    result="$(some_json_input | jp -r '$.1[1]')"

    [ "$result" = '3' ]
}

@test "returns successfully on -r option with selector" {
    some_json_input | jp -r '$'
}

@test "pretty prints a JSON stream" {
    result="$(some_json_stream | jp)"
    expected_output='{
  "a": [
    "b",
    0
  ]
}
{
  "a": [
    "ccc",
    1
  ]
}'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a single match selector for JSON stream" {
    result="$(some_json_stream | jp '$.a[0]')"
    expected_output='"b"
"ccc"'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a multiple match selector for JSON stream" {
    result="$(some_json_stream | jp '$.a[*]')"
    expected_output='"b"
0
"ccc"
1'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a single match selector for JSON stream with -r option" {
    result="$(some_json_stream | jp -r '$.a[0]')"
    expected_output='b
ccc'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a multiple match selector for JSON stream with -r option" {
    result="$(some_json_stream | jp -r '$.a[*]')"
    expected_output='b
0
ccc
1'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "allows chaining of jp calls" {
    result="$(some_json_stream | jp '$.a' | jp '$[1]')"
    expected_output='0
1'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "unwraps a JSON stream without selector" {
    result="$(primitives_json_stream | jp -r)"
    expected_output='str
42'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "prints matches separated by tabs for -t" {
    result="$(some_json_input | jp -t '$["1"][*]')"
    expected_output="$(echo -e '"2"\t3')"
    diff <(echo "$result") <(echo "$expected_output")
}

@test "allows for combining -t and -r" {
    result="$(some_json_input | jp -rt '$["1"][*]')"
    expected_output="$(echo -e '2\t3')"
    diff <(echo "$result") <(echo "$expected_output")
}

@test "serializes a stream into tabs" {
    result="$(some_json_stream | jp -t '$["a"][*]')"
    expected_output="$(echo -e '"b"\t0\n"ccc"\t1')"
    diff <(echo "$result") <(echo "$expected_output")
}

@test "fails if -t is used without selector" {
    {
        run jp -t
    } <<< ''

    [ "$status" -eq 1 ]
}

@test "allows selecting columns when combining two queries of jp" {
    result="$(some_json_stream | jp '$.a' | jp -t '$[1,0]'])"
    expected_output="$(echo -e '0\t"b"\n1\t"ccc"')"
    diff <(echo "$result") <(echo "$expected_output")
}

@test "ships an example" {
    result="$(jp --example -r "$..author")"
    expected_output='Nigel Rees
Evelyn Waugh
Herman Melville
J. R. R. Tolkien'
    diff <(echo "$result") <(echo "$expected_output")
}
