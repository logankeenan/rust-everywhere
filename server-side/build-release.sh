#!/bin/bash

cargo install cross
cross build --target aarch64-unknown-linux-gnu --release