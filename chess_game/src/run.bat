@echo off

:: Run clippy
cargo clippy
if %errorlevel% neq 0 (
    echo Clippy failed. Exiting...
    exit /b %errorlevel%
)

:: Run tests
cargo test
if %errorlevel% neq 0 (
    echo Tests failed. Exiting...
    exit /b %errorlevel%
)

:: Build the main.rs
rustc main.rs
if %errorlevel% neq 0 (
    echo Build failed. Exiting...
    exit /b %errorlevel%
)

:: Run the compiled executable
.\main.exe
