#!/bin/bash

cd ../spa
. ./scripts/release.sh

cp -r dist ../spa-server
cd ../spa-server

#cargo install cross
#cross build --target aarch64-unknown-linux-gnu --release