#!/bin/bash

. ./sqlx.sh database create
. ./sqlx.sh migrate run

cargo build --release