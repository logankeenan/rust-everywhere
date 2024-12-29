#!/bin/bash

cargo install sqlx-cli

. ./sqlx.sh database create
. ./sqlx.sh migrate run

cargo build --release