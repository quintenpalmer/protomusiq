#!/bin/bash

set -e
set -x

cargo check
cargo doc
cargo build --release

cp ../../target/release/musiqapp ~/.bin/
cp ../../target/release/musiqapp ~/storage/projects/buildexecs/musiqapp/
cp assets/Musiqapp.desktop       ~/storage/projects/buildexecs/musiqapp/

sed -e "s|%USERNAME%|$HOME|g" assets/Musiqapp.desktop > ~/.local/share/applications/Musiqapp.desktop
update-desktop-database ~/.local/share/applications/

#~/.bin/musiqapp
