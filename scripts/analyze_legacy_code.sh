#!/bin/bash

# Script de análisis de código legacy
# Encuentra todos los usos de Matrix, Vector y ComplexVector

echo "========================================"
echo "ANÁLISIS DE CÓDIGO LEGACY"
echo "========================================"
echo ""

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para contar y mostrar
count_and_show() {
    local pattern=$1
    local description=$2
    local files=$3

    echo -e "${YELLOW}${description}${NC}"

    if [ -z "$files" ]; then
        files="crates/"
    fi

    local count=$(grep -r "$pattern" $files --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | grep -v "scripts/" | wc -l)

    if [ $count -gt 0 ]; then
        echo -e "${RED}  Encontrados: $count usos${NC}"
        echo ""
        grep -rn "$pattern" $files --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | grep -v "scripts/" | head -10
        if [ $count -gt 10 ]; then
            echo "  ... y $((count - 10)) más"
        fi
    else
        echo -e "${GREEN}  ✓ No se encontraron usos${NC}"
    fi
    echo ""
}

echo "========================================"
echo "1. BÚSQUEDA DE Matrix"
echo "========================================"
echo ""

count_and_show "use.*matrix::Matrix" "Imports de Matrix:"
count_and_show "Value::Matrix" "Uso en Value enum:"
count_and_show "Matrix::" "Llamadas estáticas a Matrix:"
count_and_show "-> .*Matrix|: Matrix" "Tipos de retorno/parámetros Matrix:"

echo "========================================"
echo "2. BÚSQUEDA DE Vector (RealVector)"
echo "========================================"
echo ""

count_and_show "use.*vector::Vector" "Imports de Vector (RealVector):"
count_and_show "Vector::new\|Vector::zeros\|Vector::ones" "Construcción de Vector:"
count_and_show "to_real_vector\|from_real_vector" "Conversiones legacy Vector:"

echo "========================================"
echo "3. BÚSQUEDA DE ComplexVector"
echo "========================================"
echo ""

count_and_show "use.*ComplexVector" "Imports de ComplexVector:"
count_and_show "ComplexVector::" "Uso de ComplexVector:"
count_and_show "to_complex_vector\|from_complex_vector" "Conversiones legacy ComplexVector:"

echo "========================================"
echo "4. ARCHIVOS MÁS AFECTADOS"
echo "========================================"
echo ""

echo "Archivos con más referencias a Matrix:"
grep -r "Matrix" crates/ --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | cut -d: -f1 | sort | uniq -c | sort -rn | head -10
echo ""

echo "========================================"
echo "5. MÓDULOS POR MIGRAR"
echo "========================================"
echo ""

echo "achronyme-solver:"
find crates/achronyme-solver -name "*.rs" -type f -exec grep -l "Matrix" {} \; 2>/dev/null | wc -l
echo "  archivos afectados"

echo ""
echo "achronyme-linalg:"
find crates/achronyme-linalg -name "*.rs" -type f -exec grep -l "Matrix" {} \; 2>/dev/null | wc -l
echo "  archivos afectados"

echo ""
echo "achronyme-eval:"
find crates/achronyme-eval -name "*.rs" -type f -exec grep -l "Matrix\|Vector" {} \; 2>/dev/null | wc -l
echo "  archivos afectados"

echo ""
echo "achronyme-wasm:"
find crates/achronyme-wasm -name "*.rs" -type f -exec grep -l "Matrix" {} \; 2>/dev/null | wc -l
echo "  archivos afectados"

echo ""
echo "========================================"
echo "6. RESUMEN DE PROGRESO"
echo "========================================"
echo ""

total_matrix=$(grep -r "Matrix" crates/ --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | grep -v "scripts/" | wc -l)
total_vector=$(grep -r "use.*vector::Vector" crates/ --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | wc -l)
total_complex=$(grep -r "ComplexVector" crates/ --include="*.rs" 2>/dev/null | grep -v "docs/" | grep -v "target/" | wc -l)

total=$((total_matrix + total_vector + total_complex))

echo "Referencias totales a código legacy:"
echo "  - Matrix: $total_matrix"
echo "  - Vector: $total_vector"
echo "  - ComplexVector: $total_complex"
echo "  - TOTAL: $total"

echo ""
if [ $total -eq 0 ]; then
    echo -e "${GREEN}✓ MIGRACIÓN COMPLETA!${NC}"
else
    echo -e "${YELLOW}⚠ Quedan $total referencias por migrar${NC}"
fi

echo ""
echo "========================================"
echo "7. SIGUIENTE PASO SUGERIDO"
echo "========================================"
echo ""

if [ $total_matrix -gt 0 ]; then
    echo "Prioridad: Migrar Matrix a Tensor"
    echo "Comenzar con:"
    grep -rl "Matrix" crates/achronyme-solver --include="*.rs" 2>/dev/null | head -1
elif [ $total_vector -gt 0 ]; then
    echo "Prioridad: Eliminar Vector (RealVector)"
    echo "Archivo a limpiar:"
    grep -rl "vector::Vector" crates/ --include="*.rs" 2>/dev/null | head -1
elif [ $total_complex -gt 0 ]; then
    echo "Prioridad: Eliminar ComplexVector"
    echo "Archivo a limpiar:"
    grep -rl "ComplexVector" crates/ --include="*.rs" 2>/dev/null | head -1
else
    echo "Eliminar archivos legacy:"
    echo "  - crates/achronyme-types/src/matrix.rs"
    echo "  - crates/achronyme-types/src/vector.rs"
    echo "  - crates/achronyme-types/src/complex_vector.rs"
fi

echo ""
