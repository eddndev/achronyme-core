# 🦀 Compilar WASM - Guía Rápida

## Opciones de Compilación

### Opción 1: WSL (Recomendado)
```bash
wsl bash scripts/build-wasm.sh
```

### Opción 2: Batch (Windows)
```cmd
scripts\build-wasm.bat
```

### Opción 3: PowerShell (Windows con Rust instalado)
```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-wasm.ps1
```

### Opción 4: Directo con wasm-pack
```bash
cd crates/achronyme-wasm
wasm-pack build --target web --out-dir ../../dist --release
cd ../..
```

### Opción 5: Usando Cargo directamente
```bash
cd crates/achronyme-wasm
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/achronyme_wasm.wasm \
  --out-dir ../../dist \
  --target web
cd ../..
```

## Requisitos

- **Rust** (con target wasm32-unknown-unknown)
- **wasm-pack** (instalar con `cargo install wasm-pack`)

## Instalar Rust + wasm-pack

### Windows (PowerShell)
```powershell
# Descargar e instalar Rust
Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
.\rustup-init.exe

# Después de instalar Rust:
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### WSL/Linux
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Agregar target WASM
rustup target add wasm32-unknown-unknown

# Instalar wasm-pack
cargo install wasm-pack
```

## Verificar Instalación

```bash
rustc --version
cargo --version
wasm-pack --version
```

## Salida Esperada

Después de compilar, deberías ver en `dist/`:

```
dist/
├── achronyme_wasm.js          (25 KB)
├── achronyme_wasm.d.ts        (9.4 KB)
├── achronyme_wasm_bg.wasm     (1.2 MB)
└── achronyme_wasm_bg.wasm.d.ts (4.5 KB)
```

## Testing

Después de compilar, abre en el navegador:

```
http://localhost:3001/tests/test-sdk-v2-basic.html
```

## Troubleshooting

### Error: "wasm-pack: command not found"
```bash
cargo install wasm-pack
```

### Error: "target not installed"
```bash
rustup target add wasm32-unknown-unknown
```

### Error: "cargo: command not found" en WSL
```bash
source $HOME/.cargo/env
# O agregar a ~/.bashrc:
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

### Compilación lenta
La primera compilación es lenta (~2-5 min). Compilaciones subsecuentes son más rápidas (~10-30 seg).

## Flags Opcionales

### Debug build (más rápido de compilar, más lento de ejecutar)
```bash
wasm-pack build --target web --out-dir ../../dist --dev
```

### Con optimizaciones de tamaño
```bash
wasm-pack build --target web --out-dir ../../dist --release -- -C opt-level=z
```

### Con source maps (para debugging)
```bash
RUSTFLAGS="-C debuginfo=2" wasm-pack build --target web --out-dir ../../dist --release
```
