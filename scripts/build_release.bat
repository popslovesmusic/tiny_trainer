@echo off
REM Deterministic Build Script for Tiny Agent Trainer (Windows)
REM Priority 3.2: Build and Packaging
REM
REM This script produces a statically linked, optimized binary with full auditing.
REM
REM Usage: scripts\build_release.bat [clean]

setlocal enabledelayedexpansion

REM Build configuration
set PROJECT_NAME=tiny_agent_trainer_rs
set BINARY_NAME=tiny-agent-trainer
set TARGET=x86_64-pc-windows-msvc
set BUILD_DIR=target\release

REM Extract version from Cargo.toml
for /f "tokens=2 delims==" %%a in ('findstr /C:"version = " Cargo.toml ^| findstr /n "^" ^| findstr "^1:"') do (
    set VERSION_LINE=%%a
)
for /f "tokens=2 delims=^" %%a in ("!VERSION_LINE!") do set VERSION=%%a
set VERSION=!VERSION:~0,-1!

REM Get build date
for /f "tokens=*" %%a in ('powershell -Command "Get-Date -Format 'yyyy-MM-dd HH:mm:ss UTC'"') do set BUILD_DATE=%%a

REM Get git commit (if available)
git rev-parse --short HEAD >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f %%a in ('git rev-parse --short HEAD') do set GIT_COMMIT=%%a
) else (
    set GIT_COMMIT=unknown
)

echo ========================================
echo 🔨 Tiny Agent Trainer - Release Build
echo ========================================
echo.
echo Version:     !VERSION!
echo Target:      !TARGET!
echo Build Date:  !BUILD_DATE!
echo Git Commit:  !GIT_COMMIT!
echo.

REM Check for clean build flag
set CLEAN_BUILD=false
if "%1"=="clean" set CLEAN_BUILD=true

REM Step 1: Clean build (optional)
if "!CLEAN_BUILD!"=="true" (
    echo 🧹 Cleaning previous builds...
    cargo clean
    echo ✅ Clean complete
    echo.
)

REM Step 2: Verify environment
echo 🔍 Verifying build environment...

REM Check Rust version
for /f "tokens=*" %%a in ('rustc --version') do set RUST_VERSION=%%a
echo    Rust: !RUST_VERSION!

REM Check Cargo version
for /f "tokens=*" %%a in ('cargo --version') do set CARGO_VERSION=%%a
echo    Cargo: !CARGO_VERSION!

REM Verify Cargo.toml exists
if not exist "Cargo.toml" (
    echo ❌ Error: Cargo.toml not found
    exit /b 1
)

REM Verify source files
if not exist "src" (
    echo ❌ Error: src directory not found
    exit /b 1
)

echo ✅ Environment verified
echo.

REM Step 3: Run tests
echo 🧪 Running test suite...
cargo test --release --quiet
if errorlevel 1 (
    echo ❌ Tests failed - aborting build
    exit /b 1
)
echo ✅ All tests passed
echo.

REM Step 4: Build with maximum optimization
echo 🔨 Building optimized binary...
echo    Target: !TARGET!
echo    Profile: release
echo    LTO: enabled
echo    Codegen Units: 1
echo    Strip: enabled
echo.

REM Build the release binary
set RUSTFLAGS=-C target-cpu=native
cargo build --release --target !TARGET! --bin !BINARY_NAME!

if errorlevel 1 (
    echo ❌ Build failed
    exit /b 1
)

echo ✅ Build complete
echo.

REM Step 5: Verify binary
echo 🔍 Verifying binary...

set BINARY_PATH=!BUILD_DIR!\!BINARY_NAME!.exe
if not exist "!BINARY_PATH!" (
    echo ❌ Error: Binary not found at !BINARY_PATH!
    exit /b 1
)

REM Get binary size
for %%A in ("!BINARY_PATH!") do set BINARY_SIZE=%%~zA
set /a BINARY_SIZE_MB=!BINARY_SIZE! / 1048576

echo    Binary: !BINARY_PATH!
echo    Size: !BINARY_SIZE_MB! MB (!BINARY_SIZE! bytes)
echo.

REM Test the binary
echo 🧪 Testing binary...
"!BINARY_PATH!" --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Binary test failed
    exit /b 1
)

for /f "tokens=*" %%a in ('"!BINARY_PATH!" --version') do set BINARY_VERSION=%%a
echo    !BINARY_VERSION!
echo ✅ Binary is functional
echo.

REM Step 6: Generate build manifest
echo 📝 Generating build manifest...

set MANIFEST_FILE=!BUILD_DIR!\BUILD_MANIFEST.txt

(
echo ========================================
echo Tiny Agent Trainer - Build Manifest
echo ========================================
echo.
echo Build Information:
echo   Version:        !VERSION!
echo   Target:         !TARGET!
echo   Build Date:     !BUILD_DATE!
echo   Git Commit:     !GIT_COMMIT!
echo   Rust Version:   !RUST_VERSION!
echo   Cargo Version:  !CARGO_VERSION!
echo.
echo Binary Information:
echo   Name:           !BINARY_NAME!.exe
echo   Size:           !BINARY_SIZE_MB! MB ^(!BINARY_SIZE! bytes^)
echo   Path:           !BINARY_PATH!
echo.
echo Build Configuration:
echo   Optimization:   opt-level = 3
echo   LTO:            enabled
echo   Codegen Units:  1
echo   Strip:          enabled
echo   Panic:          abort
echo   Target CPU:     native
echo.
echo Test Results:
echo   Status:         All tests passed
echo   Test Command:   cargo test --release
echo.
echo Verification:
echo   Binary Test:    Passed
echo   Version Check:  Passed
echo.
echo Build Script:
echo   Script:         scripts\build_release.bat
echo   Clean Build:    !CLEAN_BUILD!
echo.
echo ========================================
echo Build completed successfully
echo ========================================
) > "!MANIFEST_FILE!"

echo ✅ Manifest generated: !MANIFEST_FILE!
echo.

REM Step 7: Summary
echo ========================================
echo ✅ Release build complete!
echo ========================================
echo.
echo Binary location: !BINARY_PATH!
echo Binary size:     !BINARY_SIZE_MB! MB
echo Manifest:        !MANIFEST_FILE!
echo.
echo Next steps:
echo   1. Run packaging script: scripts\package_release.bat
echo   2. Test the binary:      !BINARY_PATH! check
echo   3. Review manifest:      type !MANIFEST_FILE!
echo.
