@echo off
REM Rust 3Dゲームプロジェクトの実行スクリプト

if "%1"=="web" (
    echo Running web version...
    pushd platform\web
    trunk serve
    popd
) else (
    echo Running desktop version...
    start target\release\app_desktop.exe
)
pause