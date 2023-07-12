#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release

cp ../../target/release/musiqapp ~/.bin/

sed -e "s|%USERNAME%|$HOME|g" assets/Musiqapp.desktop > ~/.local/share/applications/Musiqapp.desktop
update-desktop-database ~/.local/share/applications/

#~/.bin/musiqapp
