
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

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

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8);
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {number} numerator_order
* @param {number} denominator_order
* @param {number} pass_band_edge
* @param {number} stop_band_edge
* @param {number} desired_group_delay
* @param {number} max_ripple
* @param {number} division_approximation_band
* @param {number} division_transition_band
* @param {number} number_of_search_points
* @param {number} max_iteration
* @param {number} weight
* @param {number} c1
* @param {number} c2
* @param {number} init_scale
* @param {number} init_a
* @param {Float64Array} normalized_angular_frequency
* @param {Float64Array} magnitude_response
* @param {Float64Array} group_delay
* @param {Float64Array} a
* @param {Float64Array} b
* @returns {number}
*/
export function iir_design_pso(numerator_order, denominator_order, pass_band_edge, stop_band_edge, desired_group_delay, max_ripple, division_approximation_band, division_transition_band, number_of_search_points, max_iteration, weight, c1, c2, init_scale, init_a, normalized_angular_frequency, magnitude_response, group_delay, a, b) {
    try {
        var ptr0 = passArrayF64ToWasm0(normalized_angular_frequency, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArrayF64ToWasm0(magnitude_response, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArrayF64ToWasm0(group_delay, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        var ptr4 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        var len4 = WASM_VECTOR_LEN;
        var ret = wasm.iir_design_pso(numerator_order, denominator_order, pass_band_edge, stop_band_edge, desired_group_delay, max_ripple, division_approximation_band, division_transition_band, number_of_search_points, max_iteration, weight, c1, c2, init_scale, init_a, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
        return ret;
    } finally {
        normalized_angular_frequency.set(getFloat64Memory0().subarray(ptr0 / 8, ptr0 / 8 + len0));
        wasm.__wbindgen_free(ptr0, len0 * 8);
        magnitude_response.set(getFloat64Memory0().subarray(ptr1 / 8, ptr1 / 8 + len1));
        wasm.__wbindgen_free(ptr1, len1 * 8);
        group_delay.set(getFloat64Memory0().subarray(ptr2 / 8, ptr2 / 8 + len2));
        wasm.__wbindgen_free(ptr2, len2 * 8);
        a.set(getFloat64Memory0().subarray(ptr3 / 8, ptr3 / 8 + len3));
        wasm.__wbindgen_free(ptr3, len3 * 8);
        b.set(getFloat64Memory0().subarray(ptr4 / 8, ptr4 / 8 + len4));
        wasm.__wbindgen_free(ptr4, len4 * 8);
    }
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_getRandomValues_11115a852729f4e8 = handleError(function(arg0, arg1, arg2) {
        getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));
    });
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_randomFillSync_a2d002fc3b8e30f7 = handleError(function(arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_self_a5f0fe5564782787 = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_static_accessor_MODULE_7f278c5446c126c8 = function() {
        var ret = module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_29e58b5f6f133563 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_crypto_d91429ea1a087f70 = function(arg0) {
        var ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_c8be2bb4fc7d8cd3 = function(arg0) {
        var ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

