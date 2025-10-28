#!/bin/bash
set -e

echo "🔨 Building Achronyme Core (WASM)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Emscripten is available
if ! command -v emcc &> /dev/null; then
    echo -e "${YELLOW}⚠️  Emscripten not found. Please install and activate emsdk:${NC}"
    echo "   git clone https://github.com/emscripten-core/emsdk.git"
    echo "   cd emsdk"
    echo "   ./emsdk install latest"
    echo "   ./emsdk activate latest"
    echo "   source ./emsdk_env.sh"
    exit 1
fi

echo -e "${BLUE}📦 Emscripten version:${NC}"
emcc --version | head -1

# Create dist directory if it doesn't exist
mkdir -p dist

# Build with emcc directly
echo -e "\n${BLUE}🔧 Compiling C++ → WASM...${NC}"
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
  -O3 \
  -std=c++17

# Print file sizes
echo -e "\n${GREEN}✅ Build complete!${NC}"
echo -e "${BLUE}📊 File sizes:${NC}"
if command -v ls &> /dev/null; then
    ls -lh dist/achronyme-core.{mjs,wasm} 2>/dev/null | awk '{print "   " $9 ": " $5}' || true
fi

echo -e "\n${GREEN}🎉 Ready to use!${NC}"
echo "   Import in JS: import AchronymeCore from './dist/achronyme-core.mjs'"
