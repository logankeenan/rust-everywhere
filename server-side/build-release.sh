#!/bin/bash

cargo clean
cargo install cross
cross build --target aarch64-unknown-linux-gnu --release