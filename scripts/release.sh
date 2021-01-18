#!/bin/bash

# Exit if any subcommand fails
set -e

# Get tag
TAG=$(cat ./zopatract_cli/Cargo.toml | grep '^version' | awk '{print $3}' | sed -e 's/"//g') && echo $TAG

# Use zopatract github bot
git config --global user.email $GH_USER

# Release on Dockerhub

## Build
docker build -t zopatract .

## Log into Dockerhub
echo $DOCKERHUB_PASS | docker login -u $DOCKERHUB_USER --password-stdin

## Release under `latest` tag
docker tag zopatract:latest zopatract/zopatract:latest
docker push zopatract/zopatract:latest
echo "Published zopatract/zopatract:latest"

## Release under $TAG tag
docker tag zopatract:latest zopatract/zopatract:$TAG
docker push zopatract/zopatract:$TAG
echo "Published zopatract/zopatract:$TAG"

# Release on Github
git tag -f latest
git tag $TAG
git push origin -f latest
git push origin $TAG

# Build zopatract js
docker build -t zopatract_js -f zopatract_js/Dockerfile .

CID=$(docker create zopatract_js)

docker cp ${CID}:/build zopatract_js/dist
docker rm -f ${CID}

cd zopatract_js/dist

# Publish zopatract_js to npmjs
chmod +x publish.sh
./publish.sh

# Publish book
MDBOOK_TAR="https://github.com/rust-lang-nursery/mdBook/releases/download/v0.2.1/mdbook-v0.2.1-x86_64-unknown-linux-gnu.tar.gz"

cd ../../zopatract_book

## Install mdbook
wget -qO- $MDBOOK_TAR | tar xvz

## Build book
./mdbook build

## Deploy to github.io
pip3 install ghp-import
git clone https://github.com/Zopatract/zopatract.github.io.git && cd zopatract.github.io
ghp-import -n -p -f -m "Documentation upload. Version:  $TAG" -b "master" -r https://zopatractbot:"$GH_TOKEN"@github.com/Zopatract/zopatract.github.io.git ../book
echo "Published book"