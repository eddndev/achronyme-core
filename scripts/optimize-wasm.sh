#!/bin/bash
set -e

echo "🚀 Optimizing WASM bundle"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

DIST_DIR="dist"
WASM_FILE="$DIST_DIR/achronyme-core.wasm"

if [ ! -f "$WASM_FILE" ]; then
    echo "❌ WASM file not found. Run 'npm run build:wasm' first."
    exit 1
fi

echo -e "${BLUE}Original size:${NC}"
ls -lh "$WASM_FILE" | awk '{print "   " $5}'

# Optimize with wasm-opt (if available)
if command -v wasm-opt &> /dev/null; then
    echo -e "\n${BLUE}🔧 Running wasm-opt...${NC}"
    wasm-opt -O3 "$WASM_FILE" -o "$WASM_FILE.opt"
    mv "$WASM_FILE.opt" "$WASM_FILE"

    echo -e "${BLUE}Optimized size:${NC}"
    ls -lh "$WASM_FILE" | awk '{print "   " $5}'
else
    echo -e "\n⚠️  wasm-opt not found (optional). Install binaryen for better optimization."
fi

# Compress with Brotli (if available)
if command -v brotli &> /dev/null; then
    echo -e "\n${BLUE}📦 Compressing with Brotli...${NC}"
    brotli -f -k "$WASM_FILE"

    echo -e "${BLUE}Brotli compressed:${NC}"
    ls -lh "$WASM_FILE.br" | awk '{print "   " $5}'
else
    echo -e "\n⚠️  brotli not found (optional). Install for better compression."
fi

echo -e "\n${GREEN}✅ Optimization complete!${NC}"
