#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release

cp ../../target/release/musiqinspector assets/

sh assets/inspector_install.sh

cp -r assets/ ~/storage/projects/buildexecs/musiqapp/
