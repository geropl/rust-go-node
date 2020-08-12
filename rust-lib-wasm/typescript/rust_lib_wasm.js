let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder } = require(String.raw`util`);

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachegetNodeBufferMemory0 = null;
function getNodeBufferMemory0() {
    if (cachegetNodeBufferMemory0 === null || cachegetNodeBufferMemory0.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory0 = Buffer.from(wasm.memory.buffer);
    }
    return cachegetNodeBufferMemory0;
}

function passStringToWasm0(arg, malloc) {

    const len = Buffer.byteLength(arg);
    const ptr = malloc(len);
    getNodeBufferMemory0().write(arg, ptr, len);
    WASM_VECTOR_LEN = len;
    return ptr;
}

const u32CvtShim = new Uint32Array(2);

const int64CvtShim = new BigInt64Array(u32CvtShim.buffer);

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
class Evaluator {

    static __wrap(ptr) {
        const obj = Object.create(Evaluator.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_evaluator_free(ptr);
    }
    /**
    * This creates a new Evaluator for the given key and domain to check against
    * @param {string} key
    * @param {string} domain
    * @returns {Evaluator}
    */
    static createFromLicenseKey(key, domain) {
        var ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.evaluator_createFromLicenseKey(ptr0, len0, ptr1, len1);
        return Evaluator.__wrap(ret);
    }
    /**
    * Validates the given license key. Returns true if the license is valid for the given domain
    * @returns {boolean}
    */
    validate() {
        var ret = wasm.evaluator_validate(this.ptr);
        return ret !== 0;
    }
    /**
    * Returns true if the given license is valid and enables the given feature
    * @param {string} feature
    * @returns {boolean}
    */
    enabled(feature) {
        var ptr0 = passStringToWasm0(feature, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.evaluator_enabled(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * Returns true if either:
    *  - the license has no restrictions on seats
    *  - the license permits at least the given number of seats
    * @param {number} seats
    * @returns {boolean}
    */
    hasEnoughSeats(seats) {
        var ret = wasm.evaluator_hasEnoughSeats(this.ptr, seats);
        return ret !== 0;
    }
    /**
    * Retrn true if:
    *  - the license permits the use of prebuilds
    *  - the accumulated time spent doing prebuilds does not exceed the one defined in the license
    * @param {BigInt} total_prebuild_time_spent_seconds
    * @returns {boolean}
    */
    canUsePrebuild(total_prebuild_time_spent_seconds) {
        int64CvtShim[0] = total_prebuild_time_spent_seconds;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.evaluator_canUsePrebuild(this.ptr, low0, high0);
        return ret !== 0;
    }
    /**
    * Returns a string representation of the license (for debugging purposes only)
    * @returns {string}
    */
    inspect() {
        try {
            const retptr = wasm.__wbindgen_export_2.value - 16;
            wasm.__wbindgen_export_2.value = retptr;
            wasm.evaluator_inspect(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_export_2.value += 16;
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
module.exports.Evaluator = Evaluator;

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

const path = require('path').join(__dirname, 'rust_lib_wasm_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

