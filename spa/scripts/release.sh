cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/notes_demo_spa.wasm --out-dir ./dist/wasm --target web