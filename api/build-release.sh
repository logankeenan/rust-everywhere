#!/bin/bash

. ./sqlx.sh database create
. ./sqlx.sh migrate run

cargo install cross
cross build --target aarch64-unknown-linux-gnu --release