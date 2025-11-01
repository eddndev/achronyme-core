#!/bin/bash

###############################################################################
# Build Complete + Performance Test - Achronyme Core
# Sistema de Handles para optimización JS ↔ WASM
###############################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  🚀 Achronyme Core - Build Complete + Performance Test       ║${NC}"
echo -e "${CYAN}║  Sistema de Handles para optimización JS ↔ WASM             ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check Emscripten
if ! command -v emcc &> /dev/null; then
    echo -e "${RED}❌ Emscripten no encontrado${NC}"
    echo ""
    echo "Activa Emscripten primero:"
    echo "  source /ruta/a/emsdk/emsdk_env.sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Emscripten encontrado"
emcc --version | head -n 1
echo ""

echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD} PASO 1/3: Compilar C++ → WASM${NC}"
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo ""
bash scripts/build-wasm.sh

echo ""
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD} PASO 2/3: Compilar TypeScript → JavaScript${NC}"
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo ""
npm run build:js

echo ""
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD} PASO 3/3: Ejecutar Performance Tests${NC}"
echo -e "${BOLD}═══════════════════════════════════════════════════════════════${NC}"
echo ""
node test-performance-heavy.mjs

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ BUILD Y TESTS COMPLETADOS EXITOSAMENTE                    ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}📝 Archivos generados:${NC}"
echo "   • dist/achronyme-core.mjs"
echo "   • dist/achronyme-core.wasm"
echo "   • dist/sdk/*.js"
echo ""
echo -e "${GREEN}🎯 Sistema listo para usar!${NC}"
