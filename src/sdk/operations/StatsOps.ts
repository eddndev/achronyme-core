/**
 * StatsOps.ts
 *
 * Statistics operations module
 * Provides sum, mean, standard deviation, min, max, etc.
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';

/**
 * Statistics operations
 *
 * Provides:
 * - Basic statistics (sum, mean, std)
 * - Min/max operations
 * - Variance and covariance
 */
export class StatsOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Basic Statistics
    // ========================================================================

    /**
     * Sum of vector elements
     *
     * @param vector Input vector
     * @returns Sum of all elements
     */
    sum(vector: Vector): number {
        const data = vector.data;
        let sum = 0;

        for (let i = 0; i < data.length; i++) {
            sum += data[i];
        }

        return sum;
    }

    /**
     * Mean (average) of vector elements
     *
     * @param vector Input vector
     * @returns Mean value
     */
    mean(vector: Vector): number {
        const sum = this.sum(vector);
        return sum / vector.length;
    }

    /**
     * Standard deviation
     *
     * @param vector Input vector
     * @param ddof Delta degrees of freedom (default: 0 for population, 1 for sample)
     * @returns Standard deviation
     */
    std(vector: Vector, ddof: number = 0): number {
        const variance = this.variance(vector, ddof);
        return Math.sqrt(variance);
    }

    /**
     * Variance
     *
     * @param vector Input vector
     * @param ddof Delta degrees of freedom (default: 0 for population, 1 for sample)
     * @returns Variance
     */
    variance(vector: Vector, ddof: number = 0): number {
        const data = vector.data;
        const n = data.length;

        if (n <= ddof) {
            throw new Error('Insufficient data for variance calculation');
        }

        const mean = this.mean(vector);
        let sumSq = 0;

        for (let i = 0; i < n; i++) {
            const diff = data[i] - mean;
            sumSq += diff * diff;
        }

        return sumSq / (n - ddof);
    }

    // ========================================================================
    // Min/Max Operations
    // ========================================================================

    /**
     * Minimum value
     *
     * @param vector Input vector
     * @returns Minimum value
     */
    min(vector: Vector): number {
        const data = vector.data;

        if (data.length === 0) {
            throw new Error('Cannot find min of empty vector');
        }

        let min = data[0];
        for (let i = 1; i < data.length; i++) {
            if (data[i] < min) {
                min = data[i];
            }
        }

        return min;
    }

    /**
     * Maximum value
     *
     * @param vector Input vector
     * @returns Maximum value
     */
    max(vector: Vector): number {
        const data = vector.data;

        if (data.length === 0) {
            throw new Error('Cannot find max of empty vector');
        }

        let max = data[0];
        for (let i = 1; i < data.length; i++) {
            if (data[i] > max) {
                max = data[i];
            }
        }

        return max;
    }

    /**
     * Index of minimum value
     *
     * @param vector Input vector
     * @returns Index of minimum value
     */
    argmin(vector: Vector): number {
        const data = vector.data;

        if (data.length === 0) {
            throw new Error('Cannot find argmin of empty vector');
        }

        let minIdx = 0;
        let minVal = data[0];

        for (let i = 1; i < data.length; i++) {
            if (data[i] < minVal) {
                minVal = data[i];
                minIdx = i;
            }
        }

        return minIdx;
    }

    /**
     * Index of maximum value
     *
     * @param vector Input vector
     * @returns Index of maximum value
     */
    argmax(vector: Vector): number {
        const data = vector.data;

        if (data.length === 0) {
            throw new Error('Cannot find argmax of empty vector');
        }

        let maxIdx = 0;
        let maxVal = data[0];

        for (let i = 1; i < data.length; i++) {
            if (data[i] > maxVal) {
                maxVal = data[i];
                maxIdx = i;
            }
        }

        return maxIdx;
    }

    // ========================================================================
    // Advanced Statistics
    // ========================================================================

    /**
     * Median value
     *
     * @param vector Input vector
     * @returns Median value
     */
    median(vector: Vector): number {
        const sorted = Array.from(vector.data).sort((a, b) => a - b);
        const n = sorted.length;

        if (n % 2 === 0) {
            // Even length: average of two middle elements
            return (sorted[n / 2 - 1] + sorted[n / 2]) / 2;
        } else {
            // Odd length: middle element
            return sorted[Math.floor(n / 2)];
        }
    }

    /**
     * Percentile
     *
     * @param vector Input vector
     * @param p Percentile (0-100)
     * @returns Value at percentile p
     */
    percentile(vector: Vector, p: number): number {
        if (p < 0 || p > 100) {
            throw new Error('Percentile must be between 0 and 100');
        }

        const sorted = Array.from(vector.data).sort((a, b) => a - b);
        const n = sorted.length;

        if (n === 0) {
            throw new Error('Cannot compute percentile of empty vector');
        }

        if (n === 1) {
            return sorted[0];
        }

        const index = (p / 100) * (n - 1);
        const lower = Math.floor(index);
        const upper = Math.ceil(index);
        const weight = index - lower;

        return sorted[lower] * (1 - weight) + sorted[upper] * weight;
    }

    /**
     * Covariance between two vectors
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @param ddof Delta degrees of freedom (default: 0)
     * @returns Covariance
     */
    cov(v1: Vector, v2: Vector, ddof: number = 0): number {
        const d1 = v1.data;
        const d2 = v2.data;

        if (d1.length !== d2.length) {
            throw new Error('Vectors must have same length');
        }

        const n = d1.length;

        if (n <= ddof) {
            throw new Error('Insufficient data for covariance calculation');
        }

        const mean1 = this.mean(v1);
        const mean2 = this.mean(v2);

        let sum = 0;
        for (let i = 0; i < n; i++) {
            sum += (d1[i] - mean1) * (d2[i] - mean2);
        }

        return sum / (n - ddof);
    }

    /**
     * Correlation coefficient (Pearson)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns Correlation coefficient (-1 to 1)
     */
    corr(v1: Vector, v2: Vector): number {
        const covariance = this.cov(v1, v2, 1);
        const std1 = this.std(v1, 1);
        const std2 = this.std(v2, 1);

        if (std1 === 0 || std2 === 0) {
            throw new Error('Cannot compute correlation with zero variance');
        }

        return covariance / (std1 * std2);
    }

    /**
     * Product of vector elements
     *
     * @param vector Input vector
     * @returns Product of all elements
     */
    prod(vector: Vector): number {
        const data = vector.data;
        let product = 1;

        for (let i = 0; i < data.length; i++) {
            product *= data[i];
        }

        return product;
    }

    /**
     * Cumulative sum
     *
     * @param vector Input vector
     * @returns Vector of cumulative sums
     */
    cumsum(vector: Vector): Vector {
        const data = vector.data;
        const result = new Float64Array(data.length);
        let sum = 0;

        for (let i = 0; i < data.length; i++) {
            sum += data[i];
            result[i] = sum;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Cumulative product
     *
     * @param vector Input vector
     * @returns Vector of cumulative products
     */
    cumprod(vector: Vector): Vector {
        const data = vector.data;
        const result = new Float64Array(data.length);
        let product = 1;

        for (let i = 0; i < data.length; i++) {
            product *= data[i];
            result[i] = product;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }
}
