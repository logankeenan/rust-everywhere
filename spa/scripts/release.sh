cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ../target/wasm32-unknown-unknown/release/spa.wasm --out-dir ./dist/wasm --target no-modules