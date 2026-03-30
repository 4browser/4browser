@echo off
REM 4 Browser Build Script (Batch)
REM This script automates the build process for 4 Browser on Windows

setlocal enabledelayedexpansion

set BUILD_TYPE=%1
if "!BUILD_TYPE!"=="" set BUILD_TYPE=release

set FEATURES=%2
if "!FEATURES!"=="" set FEATURES=

cls
echo.
echo 0x1b[36m4 Browser Build Script
echo 0x1b[36m=========================
echo.

REM Check Rust installation
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    color 0c
    echo 0x1b[31mError: Rust not installed!
    echo 0x1b[33mInstall from: https://rustup.rs/
    color 0f
    exit /b 1
)

color 0a
echo 0x1b[32m- Rust found
color 0f

rustc --version
cargo --version

echo.

REM Determine build configuration
if /i "!BUILD_TYPE!"=="debug" (
    set BUILD_FLAG=
    set BUILD_MODE=debug
    color 0e
    echo 0x1b[33mBuilding debug version...
    color 0f
) else (
    set BUILD_FLAG=--release
    set BUILD_MODE=release
    color 0e
    echo 0x1b[33mBuilding release version...
    color 0f
)

echo.
color 06
echo 0x1b[36m- Compiling...
color 0f

REM Build
if "!FEATURES!"=="" (
    call cargo build !BUILD_FLAG!
) else (
    color 06
    echo 0x1b[36m- With features: !FEATURES!
    color 0f
    call cargo build !BUILD_FLAG! --features "!FEATURES!"
)

if %errorlevel% neq 0 (
    color 0c
    echo 0x1b[31mError: Build failed!
    color 0f
    exit /b 1
)

echo.

REM Determine binary path
set BINARY_NAME=fourbrowser.exe
set BINARY_PATH=target\!BUILD_MODE!\!BINARY_NAME!

REM Check if build succeeded
if exist "!BINARY_PATH!" (
    color 0a
    echo 0x1b[32m- Build successful!
    color 0f
    echo.
    echo 0x1b[36m- Binary location:
    echo.   !BINARY_PATH!
    echo.
    echo 0x1b[32m- To run the browser:
    echo.   !BINARY_PATH!
    echo.
) else (
    color 0c
    echo 0x1b[31mError: Build failed!
    color 0f
    exit /b 1
)

color 0f
endlocal
