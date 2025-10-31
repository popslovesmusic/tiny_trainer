@echo off
REM Generate all chromatic template shaders and validate them (Windows version)

setlocal enabledelayedexpansion

echo 🎨 Generating All Chromatic Templates
echo ======================================
echo.

REM Create output directory
if not exist generated_shaders mkdir generated_shaders

REM Binary path
set BINARY=.\target\release\tiny-agent-trainer.exe

REM Check if binary exists
if not exist "%BINARY%" (
    echo ❌ Binary not found at %BINARY%
    echo    Please build first: cargo build --release
    exit /b 1
)

echo Using binary: %BINARY%
echo.

REM Generate each template
for %%o in (mix filter complement saturate) do (
    echo 🔨 Generating: chromatic_%%o.wgsl

    REM Generate shader
    %BINARY% generate ^
        --model dummy ^
        --prompt "create chromatic %%o operation" ^
        --output "generated_shaders\chromatic_%%o.wgsl"

    REM Validate shader
    echo 🔍 Validating: chromatic_%%o.wgsl
    %BINARY% validate "generated_shaders\chromatic_%%o.wgsl"

    if errorlevel 1 (
        echo ❌ Validation failed
        exit /b 1
    ) else (
        echo ✅ Valid
    )

    echo.
)

echo ======================================
echo ✅ All templates generated successfully!
echo.
echo Generated files:
dir /B generated_shaders
echo.
echo 💡 Use these shaders in your chromatic_cognition_core project
