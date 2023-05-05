#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release
cp ../../target/release/musiqapp ~/.bin/
#~/.bin/musiqapp
