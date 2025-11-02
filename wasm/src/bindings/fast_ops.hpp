#ifndef ACHRONYME_BINDINGS_FAST_OPS_HPP
#define ACHRONYME_BINDINGS_FAST_OPS_HPP

#include "../core/handle_manager.hpp"
#include "../core/value.hpp"
#include "../core/functions.hpp"
#include <cstdint>
#include <vector>

namespace achronyme {
namespace bindings {

using Handle = core::HandleManager::Handle;

/**
 * Fast Operations API - Opera directamente sobre memoria sin parsing
 *
 * Estas funciones proveen un path de alto rendimiento que evita
 * el overhead del lexer/parser/evaluator para operaciones con datos.
 */

// ============================================================================
// Vector Creation
// ============================================================================

/**
 * Crear vector desde buffer de memoria
 * @param dataPtr Puntero al array de doubles en memoria WASM
 * @param length Número de elementos
 * @return Handle al vector creado
 */
Handle createVectorFromBuffer(uintptr_t dataPtr, size_t length);

/**
 * Crear matriz desde buffer de memoria (row-major order)
 * @param dataPtr Puntero al array de doubles
 * @param rows Número de filas
 * @param cols Número de columnas
 * @return Handle a la matriz creada
 */
Handle createMatrixFromBuffer(uintptr_t dataPtr, size_t rows, size_t cols);

// ============================================================================
// Data Extraction
// ============================================================================

/**
 * Obtener puntero a los datos de un vector
 * @param handle Handle del vector
 * @param outLength Puntero donde se almacenará la longitud
 * @return Puntero a los datos (válido mientras exista el handle)
 */
uintptr_t getVectorData(Handle handle, size_t* outLength);

/**
 * Obtener longitud de un vector (sin punteros para Emscripten 4.0)
 * @param handle Handle del vector
 * @return Longitud del vector
 */
size_t getVectorLength(Handle handle);

/**
 * Obtener puntero a los datos de un vector (sin parámetros de salida para Emscripten 4.0)
 * @param handle Handle del vector
 * @return Puntero a los datos (válido mientras exista el handle)
 */
uintptr_t getVectorDataPtr(Handle handle);

/**
 * Obtener datos de una matriz
 * @param handle Handle de la matriz
 * @param outRows Puntero para número de filas
 * @param outCols Puntero para número de columnas
 * @return Puntero a los datos en row-major order
 */
uintptr_t getMatrixData(Handle handle, size_t* outRows, size_t* outCols);

/**
 * Copiar datos de un vector a un buffer
 * @param handle Handle del vector
 * @param destPtr Puntero de destino
 * @param maxLength Tamaño máximo a copiar
 * @return Número de elementos copiados
 */
size_t copyVectorToBuffer(Handle handle, uintptr_t destPtr, size_t maxLength);

// ============================================================================
// DSP Operations (Fast Path)
// ============================================================================

/**
 * FFT rápido - opera sobre handles
 * @param inputHandle Handle del vector de entrada
 * @return Handle del resultado (vector de complejos)
 */
Handle fft_fast(Handle inputHandle);

/**
 * FFT Magnitude - magnitud del espectro
 * @param inputHandle Handle del vector de entrada
 * @return Handle del resultado (vector de reales)
 */
Handle fft_mag_fast(Handle inputHandle);

/**
 * FFT Phase - fase del espectro
 * @param inputHandle Handle del vector de entrada
 * @return Handle del resultado (vector de reales)
 */
Handle fft_phase_fast(Handle inputHandle);

/**
 * IFFT rápido
 */
Handle ifft_fast(Handle inputHandle);

/**
 * Convolución rápida
 */
Handle conv_fast(Handle h1, Handle h2);

/**
 * Convolución FFT rápida
 */
Handle conv_fft_fast(Handle h1, Handle h2);

// ============================================================================
// Vector Operations (Fast Path)
// ============================================================================

/**
 * Suma de vectores element-wise
 */
Handle vadd_fast(Handle h1, Handle h2);

/**
 * Resta de vectores element-wise
 */
Handle vsub_fast(Handle h1, Handle h2);

/**
 * Multiplicación element-wise
 */
Handle vmul_fast(Handle h1, Handle h2);

/**
 * División element-wise
 */
Handle vdiv_fast(Handle h1, Handle h2);

/**
 * Escalar vector por constante
 */
Handle vscale_fast(Handle h, double scalar);

/**
 * Dot product (retorna handle a un escalar)
 */
Handle dot_fast(Handle h1, Handle h2);

/**
 * Norma de un vector
 */
Handle norm_fast(Handle h);

// ============================================================================
// Mathematical Functions (Vectorized Fast Path)
// ============================================================================

/**
 * Aplicar función sin a todo el vector
 */
Handle sin_fast(Handle h);
Handle cos_fast(Handle h);
Handle tan_fast(Handle h);
Handle exp_fast(Handle h);
Handle ln_fast(Handle h);
Handle abs_fast(Handle h);
Handle sqrt_fast(Handle h);

// ============================================================================
// Optimization Functions (Fast Path)
// ============================================================================

/**
 * Linspace - genera vector con valores equiespaciados
 * Mucho más rápido que generarlo desde JS
 */
Handle linspace_fast(double start, double end, size_t n);

/**
 * FFT shift
 */
Handle fftshift_fast(Handle h);

/**
 * Inverse FFT shift
 */
Handle ifftshift_fast(Handle h);

/**
 * FFT Spectrum completo (omega, magnitude, phase en una sola llamada)
 * @return Handle a matriz [N x 3]
 */
Handle fft_spectrum_fast(Handle signalHandle, double fs, bool shift, bool angular, double omegaRange);

// ============================================================================
// Handle Management
// ============================================================================

/**
 * Liberar un handle
 */
void releaseHandle(Handle handle);

/**
 * Verificar si un handle es válido
 */
bool isValidHandle(Handle handle);

/**
 * Obtener tipo de un handle (0=number, 1=complex, 2=vector, 3=matrix, 4=function)
 */
int getHandleType(Handle handle);

/**
 * Clonar un handle
 */
Handle cloneHandle(Handle handle);

// ============================================================================
// Integration with Evaluator
// ============================================================================

/**
 * Vincular un handle a una variable del evaluator
 * Permite mezclar fast path con expression path
 */
void bindVariableToHandle(const std::string& varName, Handle handle);

/**
 * Crear handle desde una variable existente del evaluator
 */
Handle createHandleFromVariable(const std::string& varName);

} // namespace bindings
} // namespace achronyme

#endif // ACHRONYME_BINDINGS_FAST_OPS_HPP
