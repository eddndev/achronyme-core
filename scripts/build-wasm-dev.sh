#!/bin/bash
set -e

echo "🔨 Building Achronyme Core (WASM - Development Mode)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Emscripten is available
if ! command -v emcc &> /dev/null; then
    echo -e "${YELLOW}⚠️  Emscripten not found. Please install and activate emsdk${NC}"
    exit 1
fi

# Create dist directory if it doesn't exist
mkdir -p dist

# Build with emcc directly (development mode - faster compile, no optimization)
echo -e "\n${BLUE}🔧 Compiling C++ → WASM (dev mode)...${NC}"
emcc \
  wasm/src/core/*.cpp \
  wasm/src/parser/*.cpp \
  wasm/src/bindings/main.cpp \
  -I wasm/src \
  -o dist/achronyme-core.mjs \
  -s WASM=1 \
  -s ALLOW_MEMORY_GROWTH=1 \
  -s MODULARIZE=1 \
  -s EXPORT_ES6=1 \
  -s EXPORT_NAME='AchronymeCore' \
  -s ENVIRONMENT='web,worker,node' \
  --bind \
  -fexceptions \
  -O0 \
  -g \
  -std=c++17

echo -e "\n${GREEN}✅ Dev build complete!${NC}"
echo -e "${YELLOW}⚠️  This is a development build (not optimized)${NC}"
echo -e "   Use 'npm run build:wasm' for production builds"
