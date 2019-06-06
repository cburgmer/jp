Examples from a real bash history. These may not necessarily reflect good
queries in jq, and there might be more idiomatic ways, yet those are the
solutions a non-jq expert reached for in their partial understanding of jq.

Result set of a nested document structure

    $ jq -r < contracts.json '.d.results | map(.ItemSet) | map(.results) | flatten | map(.id) | .[]'
    1
    2
    3
    4

Find a value by key in a set of documents

    $ cat items/* | jq -r '[.Id,.Status] | @tsv' | grep 1 | cut -f2
    up

Find document by a given key/value match

    $ find items -type f | xargs -n1 -I% sh -c '(jq -r .Status < % | grep up > /dev/null) && echo %'
    items/item1.json

Selecting a subset of keys

    $ cat file.json | jq '[.Category,.Location,.Status]'
    [
      "something",
      "here",
      "up"
    ]

Tabular data

    $ cat items/* | jq -r '[.Id,.Status] | join(",")'  | column -ts,
    1  up
    2  down

Tabular data is easy if the structure is flat

    $ aws ec2 describe-instances --query 'Reservations[*].Instances[*].[InstanceId,Placement.AvailabilityZone,PrivateIpAddress]' > ec2_table.json
    $ cat ec2_table.json  | jq -c '.[][]'
    ["i-0719122123fe1","eu-central-1a","188.31.1.52"]
    ["i-071612ac611aa","eu-central-1c","188.40.2.7"]

Fail on missing entry

    curl 'http://localhost:8080/auth/admin/realms/master/users?username=admin | jq -e 'map(select(.username == "admin")) | first'

## Did not work as intended

Trying to extract two values, one of them nested (`[0]` and `[1].Arn`) from the document `ec2.json`.
For multiple entries will batch values 0 and 1 instead of grouping by entry

    $ aws ec2 describe-instances --query 'Reservations[*].Instances[*].[InstanceId,IamInstanceProfile]' > ec2_nested.json'
    $ cat ec2_nested.json | jq -r '.[][][0,1]'
    i-312ab1273123e
    i-81a90136fbd71
    {
      "Arn": "arn:aws:iam::71291811:instance-profile/my-instance",
      "Id": "AIDAJAKADD"
    }
    {
      "Arn": "arn:aws:iam::71291811:instance-profile/another-instance",
      "Id": "AIFALSDSAB"
    }
