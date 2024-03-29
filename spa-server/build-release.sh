#!/bin/bash

cargo clean
rm -rf dist
rm sw.js

cd ../spa-service-worker

export APP_HOST="https://rust-everywhere-spa-server.logankeenan.com"

. ./scripts/release.sh

cp -r dist ../spa-server
cp sw.js ../spa-server
cd ../spa-server

mkdir -p dist/axum-browser-adapter/
cp node_modules/axum-browser-adapter/index.js dist/axum-browser-adapter/index.js

# Raspberry pi
#cargo install cross
#cross build --target aarch64-unknown-linux-gnu --release

# Digital Ocean
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release
