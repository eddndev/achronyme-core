@echo off
echo ╔════════════════════════════════════════════════════════════════╗
echo ║  🚀 Achronyme Core - Build Complete + Performance Test       ║
echo ║  Sistema de Handles para optimización JS ↔ WASM             ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.

REM Check Emscripten
where emcc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Emscripten no encontrado
    echo.
    echo Activa Emscripten primero:
    echo   cd C:\ruta\a\emsdk
    echo   emsdk_env.bat
    exit /b 1
)

echo ✓ Emscripten encontrado
emcc --version | findstr /C:"emcc"
echo.

echo ═══════════════════════════════════════════════════════════════
echo  PASO 1/3: Compilar C++ → WASM
echo ═══════════════════════════════════════════════════════════════
echo.
call scripts\build-wasm.bat
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ❌ Falló la compilación de WASM
    exit /b 1
)

echo.
echo ═══════════════════════════════════════════════════════════════
echo  PASO 2/3: Compilar TypeScript → JavaScript
echo ═══════════════════════════════════════════════════════════════
echo.
call npm run build:js
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ❌ Falló la compilación de TypeScript
    exit /b 1
)

echo.
echo ═══════════════════════════════════════════════════════════════
echo  PASO 3/3: Ejecutar Performance Tests
echo ═══════════════════════════════════════════════════════════════
echo.
node test-performance-heavy.mjs
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ⚠️ Tests completados con errores
    exit /b 1
)

echo.
echo ╔════════════════════════════════════════════════════════════════╗
echo ║  ✅ BUILD Y TESTS COMPLETADOS EXITOSAMENTE                    ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo 📝 Archivos generados:
echo    • dist/achronyme-core.mjs
echo    • dist/achronyme-core.wasm
echo    • dist/sdk/*.js
echo.
echo 🎯 Sistema listo para usar!
