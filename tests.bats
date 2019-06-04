#!/usr/bin/env bats

some_json_input() {
    echo '{"1": ["2", 3]}'
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
