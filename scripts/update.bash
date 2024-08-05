#!/bin/bash

# check if $PWD ends with scritps
if [[ $PWD != *scripts ]]; then
    echo "This script should be executed from the scripts directory."
    exit 1
fi

# Input, if the version should be released
read -p "Update api.yaml? (y/n) " -n 1 -r update_api

if [[ $update_api =~ ^[Yy]$ ]]; then
    ./get_openapi.bash
fi

cd ..

if [ -d "fusionauth" ]; then rm -r fusionauth; fi

mkdir fusionauth

docker image pull openapitools/openapi-generator-cli:latest
docker run --rm -v "${PWD}:/client" openapitools/openapi-generator-cli:latest \
    generate \
    -i /client/api.yaml \
    -g rust \
    -o /client/fusionauth \
    --additional-properties=pubName=fusionauth,pubHomepage=https://flexi-servers.com
rm api.yaml

cp -r fusionauth/* ./
rm -rf fusionauth
rm git_push.sh
sed -i '' 's/name = "openapi"/name = "fusionauth-rust-client"/' "Cargo.toml"
sed -i '' '/## Installation/,/```/d' "README.md"
sed -i '' '/openapi = { path = ".\/openapi" }/,/```/d' "README.md"
sed -i '' 's/license = "Apache2"/license = "Apache-2.0"/' "Cargo.toml"

cp "Cargo.toml" "Cargo.toml.bak"
awk '
    /edition = "2018"/ {
        print
        print "keywords = [ \"openapi\", \"fusionauth\"]"
        print "repository = \"https://github.com/flexi-servers/fusionauth-rust-client\""
        print "readme = \"README.md\""
        next
    }
    {print}
' "Cargo.toml.bak" > "Cargo.toml"
rm "Cargo.toml.bak"

version=$(grep -m 1 '^version =' "Cargo.toml" | sed -E 's/version = "(.*)"/\1/')
if [ -z "$version" ]; then
    echo "Unable to extract the version from Cargo.toml."
    exit 1
fi

# Inuput override version
read -p "Override version $version? (y/n) " -n 1 -r override_version

if [[ $override_version =~ ^[Yy]$ ]]; then
    read -p "Enter version: " version
    # Update version in Cargo.toml
    sed -i '' "s/^version = .*/version = \"$version\"/" "Cargo.toml"
fi

# Input, if the version should be released
read -p "Release version $version? (y/n) " -n 1 -r release_version

if [[ $release_version =~ ^[Yy]$ ]]; then
    echo "Releasing version $version."
else
    exit 0
fi


if git rev-parse "v$version" >/dev/null 2>&1; then
    echo "Git tag v$version already exists."
else
    git add .
    git commit -m "Version $version"
    git push origin master
    git tag "v$version"
    git push origin "v$version"

    cargo publish

    echo "Version $version tagged and published."
fi
