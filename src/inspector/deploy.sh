#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release

cp ../../target/release/musiqinspector ~/.bin/

cp ../../target/release/musiqinspector ~/storage/projects/buildexecs/musiqapp/assets/
