@echo off
echo 🔨 Building Achronyme Core (WASM)

REM Check if Emscripten is available
where emcc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  Emscripten not found. Please install and activate emsdk:
    echo    git clone https://github.com/emscripten-core/emsdk.git
    echo    cd emsdk
    echo    emsdk install latest
    echo    emsdk activate latest
    echo    emsdk_env.bat
    exit /b 1
)

echo 📦 Emscripten version:
emcc --version

REM Create dist directory if it doesn't exist
if not exist dist mkdir dist

REM Build with emcc directly
echo.
echo 🔧 Compiling C++ → WASM...
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
  -O3 ^
  -std=c++17

if %ERRORLEVEL% NEQ 0 (
    echo ❌ Build failed!
    exit /b 1
)

REM Print file sizes
echo.
echo ✅ Build complete!
echo 📊 File sizes:
if exist dist\achronyme-core.mjs dir /b dist\achronyme-core.mjs dist\achronyme-core.wasm | for %%f in (dist\achronyme-core.mjs dist\achronyme-core.wasm) do @echo    %%~nxf: %%~zf bytes

echo.
echo 🎉 Ready to use!
echo    Import in JS: import AchronymeCore from './dist/achronyme-core.mjs'
