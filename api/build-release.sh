#!/bin/bash

. ./sqlx.sh database create
. ./sqlx.sh migrate run

# raspberry pi
#cargo install cross
#cross build --target aarch64-unknown-linux-gnu --release

# Digital Ocean
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release
