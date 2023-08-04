#!/bin/bash

cargo clean

export APP_HOST="https://rust-everywhere-server-side.logankeenan.com"

cargo install cross
cross build --target aarch64-unknown-linux-gnu --release