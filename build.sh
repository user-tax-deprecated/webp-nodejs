#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

if ! [ -x "$(command -v napi)" ]; then
npm install -g @napi-rs/cli
if [ -x "$(command -v asdf)" ]; then
asdf reshim
fi
fi

pnpm run build
