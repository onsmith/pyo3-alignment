#!/bin/bash

# build
cargo build --manifest-path foopy/Cargo.toml
cp ./foopy/target/debug/libfoopy.so ./foopy.so

# run
python3 script.py
