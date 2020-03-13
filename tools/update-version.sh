#!/usr/bin/env bash

RELEASE_VERSION=$1
DEV_VERSION=$2

if [ "$RELEASE_VERSION" == "" -o "$DEV_VERSION" == "" ]
then
	echo "Usage: $0 RELEASE_VERSION DEV_VERSION"
	exit 0
fi

sed -i 's/^version = .*/version = "'$RELEASE_VERSION'"/' Cargo.toml
git add Cargo.toml
git commit -m "Change version to $RELEASE_VERSION for release"
git tag -a v$RELEASE_VERSION
sed -i 's/^version = .*/version = "'$DEV_VERSION'"/' Cargo.toml
git add Cargo.toml
git commit -m "Change vertion to $DEV_VERSION for development"



