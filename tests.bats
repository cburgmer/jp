#!/usr/bin/env bats

some_json_input() {
    echo '{"1": ["2", 3]}'
}

some_json_stream() {
    echo '{"a": ["b", 0]}'
    echo '{"a": ["ccc", 1]}'
}


@test "prints help" {
    jp -h | grep 'jq but with JSONPath'
}

@test "prints a version number" {
    jp --version | grep 'jp \d\.\d\.\d'
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
    result="$(some_json_input | jp '$.1.*')"

    [ "$result" = '["2",3]' ]
}

@test "fails on an invalid JSONPath selector" {
    {
        run jp INVALID
    } <<< $(echo '{}')

    [ "$status" -ne 0 ]
    [[ "$output" =~ "Unable to parse selector" ]]
}

@test "returns the matches on each line" {
    result="$(some_json_input | jp -r '$.1.*')"
    expected_output='"2"
3'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "returns the entries of an array match on each line" {
    result="$(some_json_input | jp -r '$.1')"
    expected_output='"2"
3'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "returns the matches with -r on a single result" {
    result="$(some_json_input | jp -r '$.1[1]')"

    [ "$result" = '3' ]
}

@test "returns successfully on -r option with selector" {
    some_json_input | jp -r '$'
}

@test "fails on -r option without selector" {
    run jp -r

    [ "$status" -eq 1 ]
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
    expected_output='["b",0]
["ccc",1]'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a single match selector for JSON stream with -r option" {
    result="$(some_json_stream | jp -r '$.a[0]')"
    expected_output='"b"
"ccc"'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "handles a multiple match selector for JSON stream with -r option" {
    result="$(some_json_stream | jp -r '$.a[*]')"
    expected_output='"b"
0
"ccc"
1'
    diff <(echo "$result") <(echo "$expected_output")
}

@test "allows chaining of jp calls" {
    result="$(some_json_stream | jp '$.a[*]' | jp '$[1]')"
    expected_output='0
1'
    diff <(echo "$result") <(echo "$expected_output")
}
