@echo off
echo 🔨 Building Achronyme Core (WASM - Development Mode)

REM Check if Emscripten is available
where emcc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  Emscripten not found. Please install and activate emsdk
    exit /b 1
)

REM Create dist directory if it doesn't exist
if not exist dist mkdir dist

REM Build with emcc directly (development mode - faster compile, no optimization)
echo.
echo 🔧 Compiling C++ → WASM (dev mode)...
emcc ^
  wasm/src/core/*.cpp ^
  wasm/src/parser/*.cpp ^
  wasm/src/bindings/main.cpp ^
  -I wasm/src ^
  -o dist/achronyme-core.mjs ^
  -s WASM=1 ^
  -s ALLOW_MEMORY_GROWTH=1 ^
  -s MODULARIZE=1 ^
  -s EXPORT_ES6=1 ^
  -s EXPORT_NAME='AchronymeCore' ^
  -s ENVIRONMENT='web,worker,node' ^
  --bind ^
  -fexceptions ^
  -O0 ^
  -g ^
  -std=c++17

if %ERRORLEVEL% NEQ 0 (
    echo ❌ Build failed!
    exit /b 1
)

echo.
echo ✅ Dev build complete!
echo ⚠️  This is a development build (not optimized)
echo    Use 'npm run build:wasm' for production builds
