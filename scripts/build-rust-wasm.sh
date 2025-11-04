#!/bin/bash
set -e

echo "🦀 Building Achronyme Rust → WASM"
echo ""

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}⚠️  wasm-pack not found. Installing...${NC}"
    cargo install wasm-pack
fi

echo -e "${BLUE}📦 wasm-pack version:${NC}"
wasm-pack --version

# Create dist directory
mkdir -p dist-rust

# Build with wasm-pack
echo -e "\n${BLUE}🔧 Compiling Rust → WASM...${NC}"
cd crates/achronyme-wasm
wasm-pack build \
  --target web \
  --out-dir ../../dist-rust \
  --release \
  --no-typescript

cd ../..

# Rename output to match C++ naming convention
echo -e "\n${BLUE}📝 Renaming outputs for SDK compatibility...${NC}"
cd dist-rust
mv achronyme_wasm.js achronyme-core.mjs 2>/dev/null || true
mv achronyme_wasm_bg.wasm achronyme-core.wasm 2>/dev/null || true

# Print file sizes
echo -e "\n${GREEN}✅ Build complete!${NC}"
echo -e "${BLUE}📊 File sizes:${NC}"
ls -lh achronyme-core.{mjs,wasm} 2>/dev/null | awk '{print "   " $9 ": " $5}' || true

echo -e "\n${GREEN}🎉 Ready to use!${NC}"
echo "   Rust WASM module:"
echo "   - dist-rust/achronyme-core.mjs"
echo "   - dist-rust/achronyme-core.wasm"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "   1. Test with: node tests/test-rust-wasm.mjs"
echo "   2. Update SDK to use dist-rust/ instead of dist/"
echo "   3. Run all tests to verify compatibility"
