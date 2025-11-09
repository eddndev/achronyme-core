# PowerShell script para Windows
# Análisis de código legacy para migración a Tensors

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "ANÁLISIS DE CÓDIGO LEGACY" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

function Count-Pattern($Pattern, $Description, $Path = "crates\") {

    Write-Host $Description -ForegroundColor Yellow

    $files = Get-ChildItem -Path $Path -Filter "*.rs" -Recurse -File |
             Where-Object { $_.FullName -notmatch "\\docs\\" -and
                           $_.FullName -notmatch "\\target\\" -and
                           $_.FullName -notmatch "\\scripts\\" }

    $matches = $files | Select-String -Pattern $Pattern
    $count = ($matches | Measure-Object).Count

    if ($count -gt 0) {
        Write-Host "  Encontrados: $count usos" -ForegroundColor Red
        Write-Host ""
        $matches | Select-Object -First 10 | ForEach-Object {
            Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
        }
        if ($count -gt 10) {
            Write-Host "  ... y $(($count - 10)) más"
        }
    } else {
        Write-Host "  ✓ No se encontraron usos" -ForegroundColor Green
    }
    Write-Host ""

    return $count
}

# ... rest of the script ...
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "1. BÚSQUEDA DE Matrix" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$matrix1 = Count-Pattern "use.*matrix::Matrix" "Imports de Matrix:"
$matrix2 = Count-Pattern "Value::Matrix" "Uso en Value enum:"
$matrix3 = Count-Pattern "Matrix::" "Llamadas estáticas a Matrix:"
$total_matrix = $matrix1 + $matrix2 + $matrix3

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "2. BÚSQUEDA DE Vector (RealVector)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$vector1 = Count-Pattern "use.*vector::Vector" "Imports de Vector:"
$vector2 = Count-Pattern "(Vector::new|Vector::zeros)" "Construcción de Vector:"
$total_vector = $vector1 + $vector2

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "3. BÚSQUEDA DE ComplexVector" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$complex1 = Count-Pattern "use.*ComplexVector" "Imports de ComplexVector:"
$complex2 = Count-Pattern "ComplexVector::" "Uso de ComplexVector:"
$total_complex = $complex1 + $complex2

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "4. ARCHIVOS MÁS AFECTADOS" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$affectedFiles = Get-ChildItem -Path "crates\" -Filter "*.rs" -Recurse -File |
                 Where-Object { $_.FullName -notmatch "\\docs\\" -and
                               $_.FullName -notmatch "\\target\\" } |
                 Select-String -Pattern "Matrix" |
                 Group-Object Path |
                 Sort-Object Count -Descending |
                 Select-Object -First 10

Write-Host "Archivos con más referencias a Matrix:"
foreach ($file in $affectedFiles) {
    Write-Host "  $($file.Count) - $($file.Name)"
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "5. MÓDULOS POR MIGRAR" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$solverFiles = (Get-ChildItem -Path "crates\achronyme-solver" -Filter "*.rs" -Recurse -File -ErrorAction SilentlyContinue |
                Select-String -Pattern "Matrix" |
                Select-Object -Unique Path).Count
Write-Host "achronyme-solver: $solverFiles archivos afectados"

$linalgFiles = (Get-ChildItem -Path "crates\achronyme-linalg" -Filter "*.rs" -Recurse -File -ErrorAction SilentlyContinue |
                Select-String -Pattern "Matrix" |
                Select-Object -Unique Path).Count
Write-Host "achronyme-linalg: $linalgFiles archivos afectados"

$evalFiles = (Get-ChildItem -Path "crates\achronyme-eval" -Filter "*.rs" -Recurse -File -ErrorAction SilentlyContinue |
              Select-String -Pattern "(Matrix|Vector)" |
              Select-Object -Unique Path).Count
Write-Host "achronyme-eval: $evalFiles archivos afectados"

$wasmFiles = (Get-ChildItem -Path "crates\achronyme-wasm" -Filter "*.rs" -Recurse -File -ErrorAction SilentlyContinue |
              Select-String -Pattern "Matrix" |
              Select-Object -Unique Path).Count
Write-Host "achronyme-wasm: $wasmFiles archivos afectados"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "6. RESUMEN DE PROGRESO" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$total = $total_matrix + $total_vector + $total_complex

Write-Host "Referencias totales a código legacy:"
Write-Host "  - Matrix: $total_matrix"
Write-Host "  - Vector: $total_vector"
Write-Host "  - ComplexVector: $total_complex"
Write-Host "  - TOTAL: $total"

Write-Host ""
if ($total -eq 0) {
    Write-Host "✓ MIGRACIÓN COMPLETA!" -ForegroundColor Green
} else {
    Write-Host "⚠ Quedan $total referencias por migrar" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "7. SIGUIENTE PASO SUGERIDO" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($total_matrix -gt 0) {
    Write-Host "Prioridad: Migrar Matrix a Tensor"
    $nextFile = Get-ChildItem -Path "crates\achronyme-solver" -Filter "*.rs" -Recurse -File -ErrorAction SilentlyContinue |
                Select-String -Pattern "Matrix" -List |
                Select-Object -First 1 -ExpandProperty Path
    if ($nextFile) {
        Write-Host "Comenzar con: $nextFile"
    }
} elseif ($total_vector -gt 0) {
    Write-Host "Prioridad: Eliminar Vector (RealVector)"
    $nextFile = Get-ChildItem -Path "crates\" -Filter "*.rs" -Recurse -File |
                Select-String -Pattern "vector::Vector" -List |
                Select-Object -First 1 -ExpandProperty Path
    if ($nextFile) {
        Write-Host "Archivo a limpiar: $nextFile"
    }
} elseif ($total_complex -gt 0) {
    Write-Host "Prioridad: Eliminar ComplexVector"
    $nextFile = Get-ChildItem -Path "crates\" -Filter "*.rs" -Recurse -File |
                Select-String -Pattern "ComplexVector" -List |
                Select-Object -First 1 -ExpandProperty Path
    if ($nextFile) {
        Write-Host "Archivo a limpiar: $nextFile"
    }
} else {
    Write-Host "Eliminar archivos legacy:"
    Write-Host "  - crates\achronyme-types\src\matrix.rs"
    Write-Host "  - crates\achronyme-types\src\vector.rs"
    Write-Host "  - crates\achronyme-types\src\complex_vector.rs"
}

Write-Host ""
