@echo off
echo 🦀 Building Achronyme WASM (SDK v2.0)
echo.

REM Check if in WSL
where wsl >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Using WSL...
    wsl bash scripts/build-wasm.sh
) else (
    echo ❌ WSL not found. Install WSL or use PowerShell script.
    pause
)
