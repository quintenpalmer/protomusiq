#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release

cp ../../target/release/musiqapp assets/

sh assets/app_install.sh

cp -r assets/ ~/storage/projects/buildexecs/musiqapp/

#~/.bin/musiqapp
