#!/bin/bash

cargo build --release --target=wasm32-unknown-unknown

# Needs: cargo install wasm-opt
wasm-opt \
  -O3 \
  --strip-debug \
  --strip-producers \
  --vacuum \
  --remove-unused-module-elements \
  --simplify-locals \
  --reorder-locals \
  --coalesce-locals \
  --flatten \
  --local-cse \
  -o target/wasm32-unknown-unknown/release/guest_fibonacci_opt.wasm \
  target/wasm32-unknown-unknown/release/guest_fibonacci.wasm
