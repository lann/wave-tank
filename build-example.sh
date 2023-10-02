#!/bin/bash

set -e

cargo --list | grep -E '^ *component\b' > /dev/null || {
	echo 'cargo-component may not be installed; see https://github.com/bytecodealliance/cargo-component/#installation'
	exit 1
}

cd example
cargo component build --release
mv target/wasm32-wasi/release/example.wasm ..
echo 'Moved to ./example.wasm'
