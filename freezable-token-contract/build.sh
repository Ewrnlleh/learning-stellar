#!/bin/bash
# Build script for the freezable token contract

echo "Building Freezable Token Contract..."

# Build for WASM
echo "Building for WebAssembly target..."
cargo build --target wasm32-unknown-unknown --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Contract WASM file: target/wasm32-unknown-unknown/release/freezable_token_contract.wasm"
else
    echo "❌ Build failed!"
    exit 1
fi

# Run tests on native target
echo "Running tests..."
cargo test --target x86_64-pc-windows-msvc

if [ $? -eq 0 ]; then
    echo "✅ All tests passed!"
else
    echo "❌ Some tests failed!"
    exit 1
fi

echo "Ready for deployment!"
