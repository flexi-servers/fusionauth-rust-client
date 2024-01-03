#!/bin/bash

url=https://raw.githubusercontent.com/FusionAuth/fusionauth-openapi/master/openapi.yaml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml

if [ -d "fusionauth" ]; then rm -r fusionauth; fi

mkdir fusionauth

docker run --rm -v "${PWD}:/client" openapitools/openapi-generator-cli:v7.1.0 \
    generate \
    -i /client/api.yaml \
    -g rust \
    -o /client/fusionauth \
    --additional-properties=pubName=fusionauth,pubHomepage=https://flexi-servers.com
rm api.yaml

cd fusionauth
cp -r * ../
cd ..
rm -rf fusionauth
rm git_push.sh
sed -i '' 's/name = "openapi"/name = "fusionauth-rust-client"/' "Cargo.toml"
sed -i '' '/## Installation/,/```/d' "README.md"
sed -i '' '/openapi = { path = ".\/openapi" }/,/```/d' "README.md"

