@echo off
cd /d "%~dp0"
echo Starting RustPond...
echo.
cargo run --release
echo.
echo RustPond has exited.
pause
