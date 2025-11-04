/**
 * MemoryPool.ts
 *
 * Memory pool for reusing TypedArray buffers
 * Reduces garbage collection pressure for temporary calculations
 */

/**
 * Size categories for buffer pooling
 */
enum SizeCategory {
    SMALL = 0, // < 1KB (128 floats)
    MEDIUM = 1, // < 10KB (1280 floats)
    LARGE = 2, // < 100KB (12800 floats)
    XLARGE = 3, // >= 100KB
}

/**
 * Pooled buffer with metadata
 */
interface PooledBuffer {
    buffer: Float64Array;
    size: number;
    lastUsed: number;
}

/**
 * Memory pool configuration
 */
interface MemoryPoolConfig {
    /** Maximum buffers per size category */
    maxBuffersPerCategory: number;

    /** Time before unused buffer is eligible for eviction (ms) */
    evictionTimeout: number;

    /** Enable auto-eviction of old buffers */
    autoEviction: boolean;

    /** Interval for auto-eviction checks (ms) */
    evictionInterval: number;
}

/**
 * Memory pool for TypedArray buffers
 *
 * Reduces GC pressure by reusing buffers for temporary calculations
 *
 * Features:
 * - Size-based pooling (small/medium/large/xlarge)
 * - Automatic eviction of old buffers
 * - Memory usage tracking
 * - Auto-resize strategy
 *
 * Usage:
 * ```typescript
 * const pool = new MemoryPool();
 *
 * // Acquire buffer
 * const buffer = pool.acquire(1000);
 * // ... use buffer ...
 * pool.release(buffer);
 * ```
 */
export class MemoryPool {
    private pools = new Map<SizeCategory, PooledBuffer[]>();
    private config: MemoryPoolConfig;
    private evictionTimer?: number;
    private stats = {
        acquired: 0,
        released: 0,
        hits: 0,
        misses: 0,
        evicted: 0,
    };

    constructor(config?: Partial<MemoryPoolConfig>) {
        this.config = {
            maxBuffersPerCategory: 10,
            evictionTimeout: 30_000, // 30 seconds
            autoEviction: true,
            evictionInterval: 10_000, // 10 seconds
            ...config,
        };

        // Initialize pools for each size category
        for (const category of [
            SizeCategory.SMALL,
            SizeCategory.MEDIUM,
            SizeCategory.LARGE,
            SizeCategory.XLARGE,
        ]) {
            this.pools.set(category, []);
        }

        // Start auto-eviction if enabled
        if (this.config.autoEviction) {
            this.startAutoEviction();
        }
    }

    /**
     * Acquire a buffer of at least the specified size
     *
     * @param size Minimum number of elements needed
     * @returns Float64Array buffer (may be larger than requested)
     */
    acquire(size: number): Float64Array {
        this.stats.acquired++;

        const category = this.getSizeCategory(size);
        const pool = this.pools.get(category)!;

        // Try to find a suitable buffer in the pool
        for (let i = 0; i < pool.length; i++) {
            const pooled = pool[i];
            if (pooled.size >= size) {
                // Found a suitable buffer
                pool.splice(i, 1); // Remove from pool
                this.stats.hits++;

                // Return view of the required size
                return pooled.buffer.subarray(0, size);
            }
        }

        // No suitable buffer found, allocate new one
        this.stats.misses++;
        return new Float64Array(size);
    }

    /**
     * Release a buffer back to the pool
     *
     * @param buffer Buffer to release
     */
    release(buffer: Float64Array): void {
        this.stats.released++;

        const size = buffer.length;
        const category = this.getSizeCategory(size);
        const pool = this.pools.get(category)!;

        // Check if pool is full
        if (pool.length >= this.config.maxBuffersPerCategory) {
            // Pool full, evict oldest buffer
            pool.shift();
            this.stats.evicted++;
        }

        // Add buffer to pool
        pool.push({
            buffer: buffer,
            size: size,
            lastUsed: Date.now(),
        });
    }

    /**
     * Get size category for a buffer
     */
    private getSizeCategory(size: number): SizeCategory {
        if (size < 128) return SizeCategory.SMALL;
        if (size < 1280) return SizeCategory.MEDIUM;
        if (size < 12800) return SizeCategory.LARGE;
        return SizeCategory.XLARGE;
    }

    /**
     * Start auto-eviction timer
     */
    private startAutoEviction(): void {
        this.evictionTimer = setInterval(() => {
            this.evictOldBuffers();
        }, this.config.evictionInterval) as unknown as number;
    }

    /**
     * Stop auto-eviction timer
     */
    private stopAutoEviction(): void {
        if (this.evictionTimer !== undefined) {
            clearInterval(this.evictionTimer);
            this.evictionTimer = undefined;
        }
    }

    /**
     * Evict old unused buffers
     *
     * @returns Number of buffers evicted
     */
    evictOldBuffers(): number {
        const now = Date.now();
        const threshold = now - this.config.evictionTimeout;
        let evicted = 0;

        for (const pool of this.pools.values()) {
            const filtered = pool.filter((pooled) => {
                const shouldEvict = pooled.lastUsed < threshold;
                if (shouldEvict) evicted++;
                return !shouldEvict;
            });

            pool.length = 0;
            pool.push(...filtered);
        }

        this.stats.evicted += evicted;
        return evicted;
    }

    /**
     * Clear all pools
     */
    clear(): void {
        for (const pool of this.pools.values()) {
            pool.length = 0;
        }
    }

    /**
     * Get memory pool statistics
     */
    getStats() {
        const poolSizes = new Map<SizeCategory, number>();
        const poolMemory = new Map<SizeCategory, number>();

        for (const [category, pool] of this.pools) {
            poolSizes.set(category, pool.length);

            // Calculate memory usage (bytes)
            const memory = pool.reduce((sum, p) => sum + p.size * 8, 0);
            poolMemory.set(category, memory);
        }

        const totalMemory = Array.from(poolMemory.values()).reduce(
            (sum, m) => sum + m,
            0
        );

        return {
            ...this.stats,
            hitRate: this.stats.acquired > 0
                ? (this.stats.hits / this.stats.acquired) * 100
                : 0,
            poolSizes: Object.fromEntries(poolSizes),
            poolMemory: Object.fromEntries(poolMemory),
            totalMemory,
            totalMemoryMB: totalMemory / (1024 * 1024),
        };
    }

    /**
     * Destroy pool and release all resources
     */
    destroy(): void {
        this.stopAutoEviction();
        this.clear();
    }
}
