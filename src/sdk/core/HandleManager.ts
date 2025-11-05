/**
 * HandleManager.ts
 *
 * Centralized handle management with MANUAL cleanup only
 * No automatic GC - cleanup must be done via dispose() or session.use()
 */

import type { Handle, HandleStats } from '../types';
import type { RustWASM } from './RustBindings';

// Forward declaration for Value type (will be defined in values/)
interface Value {
    handle: Handle;
    dispose(): void;
}

/**
 * Manages WASM handles with MANUAL cleanup only
 *
 * Features:
 * - Simple Map tracking of handles
 * - Manual cleanup via dispose() or session.use()
 * - Memory statistics
 */
export class HandleManager {
    private handles = new Map<Handle, Value>();
    private allocatedCount = 0;
    private freedCount = 0;

    constructor(private wasm: RustWASM) {
        // No FinalizationRegistry - cleanup is 100% manual
    }

    /**
     * Register a handle for manual cleanup tracking
     */
    register(handle: Handle, value: Value): void {
        this.handles.set(handle, value);
        this.allocatedCount++;
    }

    /**
     * Release a handle manually
     *
     * Called when dispose() is invoked or when session.use() exits
     */
    release(handle: Handle): void {
        if (!this.handles.has(handle)) {
            return; // Already released
        }

        // Remove from tracking
        this.handles.delete(handle);

        // Release in WASM
        try {
            this.wasm.releaseHandle(handle);
            this.freedCount++;
        } catch (error) {
            console.warn(`Failed to release handle ${handle}:`, error);
        }
    }

    /**
     * Get Value from handle
     */
    get(handle: Handle): Value | undefined {
        return this.handles.get(handle);
    }

    /**
     * Check if handle is valid
     */
    has(handle: Handle): boolean {
        return this.handles.has(handle);
    }

    /**
     * Get memory statistics
     */
    getStats(): HandleStats {
        return {
            allocated: this.allocatedCount,
            freed: this.freedCount,
            active: this.handles.size,
            leaked: 0, // Not applicable with manual cleanup
        };
    }

    /**
     * Garbage collect all handles tracked by this manager.
     * This is the primary mechanism for session-based cleanup.
     *
     * @returns Number of handles freed
     */
    gc(): number {
        const handles = Array.from(this.handles.keys());
        const count = handles.length;
        for (const handle of handles) {
            this.release(handle);
        }
        return count;
    }

    /**
     * Reset statistics (for testing)
     */
    resetStats(): void {
        this.allocatedCount = 0;
        this.freedCount = 0;
    }
}
