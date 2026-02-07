#!/bin/bash


if [[ $PWD != *scripts ]]; then
    echo "This script should be executed from the scripts directory."
    exit 1
fi


cd ..

url=https://raw.githubusercontent.com/FusionAuth/fusionauth-openapi/master/openapi.yaml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml
