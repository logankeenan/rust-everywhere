#!/bin/bash

cd ../spa
. ./scripts/release.sh

cp -r dist ../spa-server
cd ../spa-server

mkdir -p dist/axum-browser-adapter/
cp node_modules/axum-browser-adapter/index.js dist/axum-browser-adapter/index.js

#cargo install cross
#cross build --target aarch64-unknown-linux-gnu --release