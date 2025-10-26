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

# Create build directory
BUILD_DIR="wasm/build"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configure with CMake
echo -e "\n${BLUE}📋 Configuring with CMake...${NC}"
emcmake cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_SIMD=OFF

# Build
echo -e "\n${BLUE}🔧 Compiling C++ → WASM...${NC}"
emmake make -j$(nproc 2>/dev/null || echo 4)

# Create dist directory if it doesn't exist
cd ../..
mkdir -p dist

# Copy artifacts
echo -e "\n${BLUE}📦 Copying artifacts to dist/...${NC}"
cp "$BUILD_DIR/achronyme-core.js" dist/
cp "$BUILD_DIR/achronyme-core.wasm" dist/

# Print file sizes
echo -e "\n${GREEN}✅ Build complete!${NC}"
echo -e "${BLUE}📊 File sizes:${NC}"
ls -lh dist/achronyme-core.{js,wasm} | awk '{print "   " $9 ": " $5}'

echo -e "\n${GREEN}🎉 Ready to use!${NC}"
echo "   Import in JS: import AchronymeCore from './dist/achronyme-core.js'"
