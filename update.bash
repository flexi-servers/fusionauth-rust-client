#!/bin/bash


url=https://raw.githubusercontent.com/FusionAuth/fusionauth-openapi/master/openapi.yaml
echo "Fetching swagger from $url"

curl "$url" --max-time 5 > api.yaml


API_VERSION=$(grep 'version:' "api.yaml" | awk '{print $2}')
CARGO_VERSION=$(grep -m 1 '^version =' "Cargo.toml" | sed -E 's/version = "(.*)"/\1/')


if [ "$API_VERSION" != "$CARGO_VERSION" ]; then
    echo "The versions match."
    exit 0
fi

if [ -d "fusionauth" ]; then rm -r fusionauth; fi

mkdir fusionauth

docker run --rm -v "${PWD}:/client" openapitools/openapi-generator-cli:v7.1.0 \
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

if git rev-parse "v$version" >/dev/null 2>&1; then
    echo "Git tag v$version already exists."
else
    git add .
    git commit -m "Version $version"
    git push origin master
    git tag "v$version"
    git push origin "v$version"

    # cargo publish

    echo "Version $version tagged and published."
fi
