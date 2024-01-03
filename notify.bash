#!/bin/bash

# Define variables
REPO="flexi-servers/fusionauth-rust-client"  # Replace with your GitHub username and repository
ASSIGNEE="devmaxde"  # Replace with your GitHub username


url=https://raw.githubusercontent.com/FusionAuth/fusionauth-openapi/master/openapi.yaml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml

API_VERSION=$(grep 'version:' "api.yaml" | awk '{print $2}')
CARGO_VERSION=$(grep -m 1 '^version =' "Cargo.toml" | sed -E 's/version = "(.*)"/\1/')


if [ "$API_VERSION" != "$CARGO_VERSION" ]; then
    curl -X POST -H "Authorization: token $GITHUB_TOKEN" \
         -d "{\"title\": \"New Version $API_VERSION Available\", \"body\": \"A new version of the software is available: $API_VERSION.\", \"assignees\": [\"$ASSIGNEE\"]}" \
         "https://api.github.com/repos/$REPO/issues"
fi