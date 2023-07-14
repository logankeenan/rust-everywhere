cargo build --target wasm32-unknown-unknown
wasm-bindgen ../target/wasm32-unknown-unknown/debug/spa.wasm --out-dir ./dist/wasm --target no-modules