/**
 * Value.ts
 *
 * Abstract base class for all Achronyme values
 * Provides common functionality for handle management and disposal
 */

import type { Handle, ValueMetadata } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Abstract base class for all values
 *
 * All values in Achronyme are backed by WASM handles
 * This class provides common functionality for:
 * - Handle management
 * - Disposal
 * - Metadata tracking
 * - Disposed state checking
 */
export abstract class Value {
    protected _handle: Handle;
    protected _session: AchronymeSession;
    protected _disposed = false;
    protected _metadata: ValueMetadata;

    constructor(session: AchronymeSession, handle: Handle, type: ValueMetadata['type']) {
        this._session = session;
        this._handle = handle;
        this._metadata = {
            handle,
            usedFastPath: true,
            createdAt: Date.now(),
            type,
        };

        // Track in session for auto-cleanup
        this._session.track(this);
    }

    /**
     * Get the WASM handle
     */
    get handle(): Handle {
        this.checkDisposed();
        return this._handle;
    }

    /**
     * Get value metadata
     */
    get metadata(): Readonly<ValueMetadata> {
        return this._metadata;
    }

    /**
     * Check if value has been disposed
     */
    get isDisposed(): boolean {
        return this._disposed;
    }

    /**
     * Dispose this value and release WASM resources
     *
     * After calling dispose(), this value cannot be used anymore
     * All methods will throw an error
     *
     * Note: With session-based management, you usually don't need
     * to call this manually - values are auto-cleaned up
     */
    dispose(): void {
        if (this._disposed) return;

        try {
            // Release handle in WASM
            this._session.handleManager.release(this._handle);

            // Untrack from session
            this._session.untrack(this);

            this._disposed = true;
        } catch (error) {
            console.warn('Error disposing value:', error);
        }
    }

    /**
     * Check if disposed and throw if so
     */
    protected checkDisposed(): void {
        if (this._disposed) {
            throw new Error(
                `Cannot use disposed ${this._metadata.type}. ` +
                `Value was disposed at ${new Date(this._metadata.createdAt).toISOString()}`
            );
        }
    }

    /**
     * Get the session this value belongs to
     */
    protected get session(): AchronymeSession {
        return this._session;
    }

    /**
     * Get the WASM module
     */
    protected get wasm() {
        return this._session.wasm;
    }

    /**
     * Abstract method: Convert to string representation
     */
    abstract toString(): string;

    /**
     * Abstract method: Get the underlying data
     */
    abstract toArray(): number[] | number[][];
}
