#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

git add -A
git commit -m.
npm version patch
git push --follow-tags

