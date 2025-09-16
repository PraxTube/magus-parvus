#!/bin/bash
# It is required to have itch `butler` installed.

binary="magus-parvus"
itch_target="Rancic/magus-parvus"
tag=$"v$(grep -m 1 '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"

rm -rf tmp_wasm
cp -r wasm tmp_wasm

cargo build --target wasm32-unknown-unknown --profile wasm
wasm-bindgen --no-typescript --out-name bevy_game --out-dir tmp_wasm --target web target/wasm32-unknown-unknown/wasm/$binary.wasm

cp -r assets tmp_wasm/
cd tmp_wasm
zip --recurse-paths ../$binary.zip .
cd ..
rm -rf tmp_wasm

butler push \
  --fix-permissions \
  --userversion=$tag \
  $binary.zip \
  $itch_target:wasm

rm $binary.zip
