#!/bin/bash
set -e

echo "🦀 Building Achronyme WASM (SDK v2.0)"
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build with wasm-pack
echo "🔧 Compiling Rust → WASM..."
cd crates/achronyme-wasm
wasm-pack build \
  --target bundler \
  --out-dir ../../dist \
  --release

cd ../..

echo ""
echo "✅ Build complete!"
echo "📊 Output:"
ls -lh dist/achronyme_wasm* 2>/dev/null | awk '{print "   " $9 ": " $5}'

echo ""
echo "🎉 Ready to test!"
echo "   Open: http://localhost:3001/tests/test-sdk-v2-basic.html"
