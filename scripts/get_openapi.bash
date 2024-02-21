#!/bin/bash
cd ..

url=https://raw.githubusercontent.com/FusionAuth/fusionauth-openapi/master/openapi.yaml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml