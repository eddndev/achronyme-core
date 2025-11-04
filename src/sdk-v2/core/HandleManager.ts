/**
 * HandleManager.ts
 *
 * Centralized handle management with automatic garbage collection
 * Uses WeakRef + FinalizationRegistry for auto-cleanup
 */

import type { Handle, HandleStats } from '../types';
import type { RustWASM } from './RustBindings';

// Forward declaration for Value type (will be defined in values/)
interface Value {
    handle: Handle;
    dispose(): void;
}

/**
 * Manages WASM handles with automatic cleanup
 *
 * Features:
 * - WeakRef tracking of Value objects
 * - FinalizationRegistry for auto-cleanup when GC'd
 * - Memory leak detection
 * - Force GC capability
 */
export class HandleManager {
    private handles = new Map<Handle, WeakRef<Value>>();
    private registry: FinalizationRegistry<Handle>;
    private allocatedCount = 0;
    private freedCount = 0;

    constructor(private wasm: RustWASM) {
        // Auto-release handles when Value is garbage collected
        this.registry = new FinalizationRegistry((handle: Handle) => {
            this.release(handle);
        });
    }

    /**
     * Register a handle with auto-cleanup
     *
     * When the Value object is garbage collected, the handle
     * will be automatically released in WASM memory
     */
    register(handle: Handle, value: Value): void {
        // Store weak reference to value
        this.handles.set(handle, new WeakRef(value));

        // Register for finalization
        // When value is GC'd, handle will be released
        this.registry.register(value, handle, value);

        this.allocatedCount++;
    }

    /**
     * Release a handle
     *
     * This is called automatically when Value is GC'd,
     * or manually when dispose() is called
     */
    release(handle: Handle): void {
        const ref = this.handles.get(handle);
        if (!ref) return;

        // Unregister from finalization (if not already GC'd)
        const value = ref.deref();
        if (value) {
            this.registry.unregister(value);
        }

        // Release in WASM
        try {
            this.wasm.releaseHandle(handle);
            this.freedCount++;
        } catch (error) {
            console.warn(`Failed to release handle ${handle}:`, error);
        }

        this.handles.delete(handle);
    }

    /**
     * Get Value from handle (if still alive)
     *
     * Returns undefined if:
     * - Handle not found
     * - Value has been garbage collected
     */
    get(handle: Handle): Value | undefined {
        const ref = this.handles.get(handle);
        return ref?.deref();
    }

    /**
     * Check if handle is valid and alive
     */
    has(handle: Handle): boolean {
        const ref = this.handles.get(handle);
        if (!ref) return false;
        return ref.deref() !== undefined;
    }

    /**
     * Get memory statistics
     *
     * Use this to detect memory leaks:
     * - leaked > 0 indicates potential memory leaks
     * - active should decrease over time as values are disposed
     */
    getStats(): HandleStats {
        let activeHandles = 0;

        // Count handles with live Value objects
        for (const ref of this.handles.values()) {
            if (ref.deref()) {
                activeHandles++;
            }
        }

        return {
            allocated: this.allocatedCount,
            freed: this.freedCount,
            active: activeHandles,
            leaked: this.allocatedCount - this.freedCount - activeHandles,
        };
    }

    /**
     * Force cleanup of all dead handles
     *
     * Iterates through all handles and releases those
     * whose Value objects have been garbage collected
     *
     * @returns Number of handles cleaned up
     */
    gc(): number {
        let cleaned = 0;

        // Find all handles with GC'd values
        const toRelease: Handle[] = [];
        for (const [handle, ref] of this.handles) {
            if (!ref.deref()) {
                toRelease.push(handle);
            }
        }

        // Release them
        for (const handle of toRelease) {
            this.release(handle);
            cleaned++;
        }

        return cleaned;
    }

    /**
     * Release all handles (emergency cleanup)
     *
     * Use this when shutting down or in error recovery
     */
    releaseAll(): void {
        const handles = Array.from(this.handles.keys());
        for (const handle of handles) {
            this.release(handle);
        }
    }

    /**
     * Reset statistics (for testing)
     */
    resetStats(): void {
        this.allocatedCount = 0;
        this.freedCount = 0;
    }
}
