@echo off
echo Building Freezable Token Contract...

REM Build for WASM
echo Building for WebAssembly target...
cargo build --target wasm32-unknown-unknown --release

REM Check if build was successful
if %ERRORLEVEL% EQU 0 (
    echo ✅ Build successful!
    echo Contract WASM file: target\wasm32-unknown-unknown\release\freezable_token_contract.wasm
) else (
    echo ❌ Build failed!
    exit /b 1
)

echo Ready for deployment!
echo See DEPLOYMENT.md for deployment instructions.
