#!/bin/bash

cargo clean
rm -rf dist
rm sw.js

cd ../spa

export APP_HOST="https://rust-everywhere-spa-server.logankeenan.com"

. ./scripts/release.sh

cp -r dist ../spa-server
cp sw.js ../spa-server
cd ../spa-server

mkdir -p dist/axum-browser-adapter/
cp node_modules/axum-browser-adapter/index.js dist/axum-browser-adapter/index.js

cargo install cross
cross build --target aarch64-unknown-linux-gnu --release