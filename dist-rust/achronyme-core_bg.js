let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * @param {number} n
 * @returns {number}
 */
export function identity_js(n) {
    const ret = wasm.identity_js(n);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @param {number} tolerance
 * @returns {boolean}
 */
export function is_symmetric_js(handle, tolerance) {
    const ret = wasm.is_symmetric_js(handle, tolerance);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {number} handle
 */
export function releaseHandle(handle) {
    wasm.releaseHandle(handle);
}

/**
 * @param {number} ptr
 */
export function _free(ptr) {
    wasm._free(ptr);
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function tan_fast(handle) {
    const ret = wasm.tan_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * Resetea el evaluador y libera todos los handles
 */
export function reset() {
    wasm.reset();
}

/**
 * @param {number} start
 * @param {number} end
 * @param {number} n
 * @returns {number}
 */
export function linspace_fast(start, end, n) {
    const ret = wasm.linspace_fast(start, end, n);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @param {number} length_ptr
 * @returns {number}
 */
export function getVectorData(handle, length_ptr) {
    const ret = wasm.getVectorData(handle, length_ptr);
    return ret >>> 0;
}

/**
 * @param {number} data_ptr
 * @param {number} len
 * @returns {number}
 */
export function createVectorFromBuffer(data_ptr, len) {
    const ret = wasm.createVectorFromBuffer(data_ptr, len);
    return ret >>> 0;
}

/**
 * @param {number} size
 * @returns {number}
 */
export function _malloc(size) {
    const ret = wasm._malloc(size);
    return ret >>> 0;
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function sin_fast(handle) {
    const ret = wasm.sin_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function abs_fast(handle) {
    const ret = wasm.abs_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {SVDResult}
 */
export function svd_decomposition_js(handle) {
    const ret = wasm.svd_decomposition_js(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return SVDResult.__wrap(ret[0]);
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function fft_fast(handle) {
    const ret = wasm.fft_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {LUResult}
 */
export function lu_decomposition_js(handle) {
    const ret = wasm.lu_decomposition_js(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return LUResult.__wrap(ret[0]);
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function sqrt_fast(handle) {
    const ret = wasm.sqrt_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function cholesky_decomposition_js(handle) {
    const ret = wasm.cholesky_decomposition_js(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function exp_fast(handle) {
    const ret = wasm.exp_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} data_ptr
 * @param {number} rows
 * @param {number} cols
 * @returns {number}
 */
export function createMatrixFromBuffer(data_ptr, rows, cols) {
    const ret = wasm.createMatrixFromBuffer(data_ptr, rows, cols);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @param {number} max_iterations
 * @param {number} tolerance
 * @returns {number}
 */
export function qr_eigenvalues_js(handle, max_iterations, tolerance) {
    const ret = wasm.qr_eigenvalues_js(handle, max_iterations, tolerance);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function cos_fast(handle) {
    const ret = wasm.cos_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
 * Evalua una expresión y retorna el resultado como string
 * @param {string} expression
 * @returns {string}
 */
export function _eval(expression) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passStringToWasm0(expression, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm._eval(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function ln_fast(handle) {
    const ret = wasm.ln_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * @param {number} handle
 * @returns {boolean}
 */
export function is_positive_definite_js(handle) {
    const ret = wasm.is_positive_definite_js(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {number} handle
 * @returns {QRResult}
 */
export function qr_decomposition_js(handle) {
    const ret = wasm.qr_decomposition_js(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return QRResult.__wrap(ret[0]);
}

/**
 * @param {number} handle
 * @param {number} max_iterations
 * @param {number} tolerance
 * @returns {PowerIterationResult}
 */
export function power_iteration_js(handle, max_iterations, tolerance) {
    const ret = wasm.power_iteration_js(handle, max_iterations, tolerance);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return PowerIterationResult.__wrap(ret[0]);
}

/**
 * @param {number} handle
 * @param {number} max_iterations
 * @param {number} tolerance
 * @returns {EigenResult}
 */
export function eigen_symmetric_js(handle, max_iterations, tolerance) {
    const ret = wasm.eigen_symmetric_js(handle, max_iterations, tolerance);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return EigenResult.__wrap(ret[0]);
}

/**
 * @param {string} name
 * @param {number} handle
 */
export function bindVariableToHandle(name, handle) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.bindVariableToHandle(ptr0, len0, handle);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} handle
 * @returns {number}
 */
export function fft_mag_fast(handle) {
    const ret = wasm.fft_mag_fast(handle);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

const EigenResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_eigenresult_free(ptr >>> 0, 1));

export class EigenResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EigenResult.prototype);
        obj.__wbg_ptr = ptr;
        EigenResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EigenResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eigenresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get eigenvalues() {
        const ret = wasm.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get eigenvectors() {
        const ret = wasm.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) EigenResult.prototype[Symbol.dispose] = EigenResult.prototype.free;

const LUResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_luresult_free(ptr >>> 0, 1));

export class LUResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LUResult.prototype);
        obj.__wbg_ptr = ptr;
        LUResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LUResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_luresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get L() {
        const ret = wasm.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get U() {
        const ret = wasm.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get P() {
        const ret = wasm.__wbg_get_luresult_P(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) LUResult.prototype[Symbol.dispose] = LUResult.prototype.free;

const PowerIterationResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_poweriterationresult_free(ptr >>> 0, 1));

export class PowerIterationResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PowerIterationResult.prototype);
        obj.__wbg_ptr = ptr;
        PowerIterationResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PowerIterationResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_poweriterationresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get eigenvalue() {
        const ret = wasm.__wbg_get_poweriterationresult_eigenvalue(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get eigenvector() {
        const ret = wasm.__wbg_get_poweriterationresult_eigenvector(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) PowerIterationResult.prototype[Symbol.dispose] = PowerIterationResult.prototype.free;

const QRResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_qrresult_free(ptr >>> 0, 1));

export class QRResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(QRResult.prototype);
        obj.__wbg_ptr = ptr;
        QRResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        QRResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_qrresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get Q() {
        const ret = wasm.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get R() {
        const ret = wasm.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) QRResult.prototype[Symbol.dispose] = QRResult.prototype.free;

const SVDResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_svdresult_free(ptr >>> 0, 1));

export class SVDResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SVDResult.prototype);
        obj.__wbg_ptr = ptr;
        SVDResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SVDResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svdresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get U() {
        const ret = wasm.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get S() {
        const ret = wasm.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get V() {
        const ret = wasm.__wbg_get_luresult_P(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) SVDResult.prototype[Symbol.dispose] = SVDResult.prototype.free;

export function __wbg___wbindgen_throw_b855445ff6a94295(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

