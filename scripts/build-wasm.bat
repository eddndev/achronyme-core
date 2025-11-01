@echo off
echo 🔨 Building Achronyme Core (WASM) - Con sistema de Handles

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
echo    Incluye: Handle Manager + Fast Operations API
echo.
emcc ^
  wasm/src/core/constants.cpp ^
  wasm/src/core/complex.cpp ^
  wasm/src/core/vector.cpp ^
  wasm/src/core/matrix.cpp ^
  wasm/src/core/function.cpp ^
  wasm/src/core/functions.cpp ^
  wasm/src/core/functions_dsp.cpp ^
  wasm/src/core/functions_hof.cpp ^
  wasm/src/core/value.cpp ^
  wasm/src/core/handle_manager.cpp ^
  wasm/src/parser/lexer.cpp ^
  wasm/src/parser/parser.cpp ^
  wasm/src/parser/evaluator.cpp ^
  wasm/src/bindings/main.cpp ^
  wasm/src/bindings/fast_ops.cpp ^
  -I wasm/src ^
  -o dist/achronyme-core.mjs ^
  -s WASM=1 ^
  -s ALLOW_MEMORY_GROWTH=1 ^
  -s MAXIMUM_MEMORY=2GB ^
  -s INITIAL_MEMORY=64MB ^
  -s MODULARIZE=1 ^
  -s EXPORT_ES6=1 ^
  -s EXPORT_NAME='AchronymeCore' ^
  -s ENVIRONMENT='web,worker,node' ^
  -s EXPORTED_RUNTIME_METHODS='["HEAPF64","HEAPU32","HEAP8"]' ^
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
echo.
echo 📝 Next steps:
echo    1. npm run build:js
echo    2. node test-performance-heavy.mjs
