# PowerShell script to build WASM
Write-Host "🦀 Building Achronyme WASM (SDK v2.0)" -ForegroundColor Cyan
Write-Host ""

# Check if wasm-pack is installed
$wasmPackPath = Get-Command wasm-pack -ErrorAction SilentlyContinue
if (-not $wasmPackPath) {
    Write-Host "❌ wasm-pack not found." -ForegroundColor Red
    Write-Host "Install with: cargo install wasm-pack" -ForegroundColor Yellow
    exit 1
}

# Build with wasm-pack
Write-Host "🔧 Compiling Rust → WASM..." -ForegroundColor Blue
Push-Location crates\achronyme-wasm

wasm-pack build `
  --target web `
  --out-dir ..\..\dist `
  --release

Pop-Location

Write-Host ""
Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host "📊 Output:" -ForegroundColor Blue
Get-ChildItem dist\achronyme_wasm* | ForEach-Object {
    Write-Host "   $($_.Name): $([math]::Round($_.Length/1KB, 2)) KB"
}

Write-Host ""
Write-Host "🎉 Ready to test!" -ForegroundColor Green
Write-Host "   Open: http://localhost:3001/tests/test-sdk-v2-basic.html"
