#!/usr/bin/bash

cd ouroboros_macro
cargo publish
sleep 30
cd ../ouroboros
cargo publish
sleep 30
cd ../examples
cargo publish
