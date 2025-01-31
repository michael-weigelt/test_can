#!/bin/bash

export RUSTFLAGS="--remap-path-prefix $(readlink -f $(dirname ${0}))=/build --remap-path-prefix ${CARGO_HOME}=/cargo"
cargo rustc -p test_can --crate-type=cdylib --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/test_can.wasm test_can.wasm

