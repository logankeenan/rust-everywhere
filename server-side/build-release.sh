#!/bin/bash

cargo clean

export APP_HOST="https://rust-everywhere-server-side.logankeenan.com"

# Raspberry pi
#cargo install cross
#cross build --target aarch64-unknown-linux-gnu --release

# Digital Ocean
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release
