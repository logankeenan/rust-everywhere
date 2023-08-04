#!/bin/bash

export APP_HOST="http://localhost:4000"
export API_HOST="http://localhost:3000"

cargo build --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/spa.wasm --out-dir ./dist/wasm --target no-modules

cp -r node_modules/axum-browser-adapter dist/