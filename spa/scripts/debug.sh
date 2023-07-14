cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/notes_demo_spa.wasm --out-dir ./dist/wasm --target web