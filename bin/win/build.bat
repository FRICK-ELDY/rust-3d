@echo off
REM Rust 3Dゲームプロジェクトのビルドスクリプト

REM デスクトップ版ビルド
echo Building desktop...
cargo build --release --manifest-path platform\desktop\Cargo.toml
if errorlevel 1 (
    echo Desktop build failed.
    exit /b 1
)

REM Web(WASM)版ビルド（Trunk使用を想定）
echo Building web (wasm)...
pushd platform\web
trunk build --release
if errorlevel 1 (
    echo Web build failed.
    popd
    exit /b 1
)
popd

echo Build completed successfully.
pause
