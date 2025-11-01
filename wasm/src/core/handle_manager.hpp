#ifndef ACHRONYME_CORE_HANDLE_MANAGER_HPP
#define ACHRONYME_CORE_HANDLE_MANAGER_HPP

#include "value.hpp"
#include <map>
#include <memory>
#include <stdexcept>
#include <cstdint>

namespace achronyme {
namespace core {

/**
 * HandleManager - Sistema de gestión de memoria con handles
 *
 * Permite operaciones de alto rendimiento sin parsing al mantener
 * valores en memoria y operar sobre ellos mediante handles (IDs).
 *
 * Características:
 * - Zero-copy para datos grandes
 * - Gestión automática de lifetime
 * - Compatible con sistema de variables del evaluator
 */
class HandleManager {
public:
    using Handle = int32_t;
    static constexpr Handle INVALID_HANDLE = -1;

    HandleManager() : nextHandle_(1) {}
    ~HandleManager() { clear(); }

    // No permitir copia (singleton pattern)
    HandleManager(const HandleManager&) = delete;
    HandleManager& operator=(const HandleManager&) = delete;

    /**
     * Crear handle desde un Value
     * @return Handle único para el valor
     */
    Handle create(const Value& value) {
        Handle handle = nextHandle_++;
        values_[handle] = std::make_shared<Value>(value);
        return handle;
    }

    /**
     * Crear handle desde un Value (move semantics)
     */
    Handle create(Value&& value) {
        Handle handle = nextHandle_++;
        values_[handle] = std::make_shared<Value>(std::move(value));
        return handle;
    }

    /**
     * Obtener valor por handle
     * @throws std::runtime_error si el handle no existe
     */
    Value& get(Handle handle) {
        auto it = values_.find(handle);
        if (it == values_.end()) {
            throw std::runtime_error("Invalid handle: " + std::to_string(handle));
        }
        return *(it->second);
    }

    const Value& get(Handle handle) const {
        auto it = values_.find(handle);
        if (it == values_.end()) {
            throw std::runtime_error("Invalid handle: " + std::to_string(handle));
        }
        return *(it->second);
    }

    /**
     * Verificar si un handle es válido
     */
    bool isValid(Handle handle) const {
        return values_.find(handle) != values_.end();
    }

    /**
     * Liberar un handle
     * @return true si se liberó, false si no existía
     */
    bool release(Handle handle) {
        return values_.erase(handle) > 0;
    }

    /**
     * Obtener número de handles activos
     */
    size_t count() const {
        return values_.size();
    }

    /**
     * Limpiar todos los handles
     */
    void clear() {
        values_.clear();
    }

    /**
     * Clonar un handle (crea una copia del valor)
     */
    Handle clone(Handle handle) {
        const Value& original = get(handle);
        return create(original);
    }

private:
    std::map<Handle, std::shared_ptr<Value>> values_;
    Handle nextHandle_;
};

// Instancia global del manager (singleton)
extern HandleManager globalHandleManager;

} // namespace core
} // namespace achronyme

#endif // ACHRONYME_CORE_HANDLE_MANAGER_HPP