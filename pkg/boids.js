let wasm;

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
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

let WASM_VECTOR_LEN = 0;

const WasmWorldFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmworld_free(ptr >>> 0, 1));

/**
 * The main simulation world exposed to JavaScript
 */
export class WasmWorld {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmWorldFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmworld_free(ptr, 0);
    }
    /**
     * Add an obstacle at position
     * @param {number} x
     * @param {number} y
     */
    add_obstacle(x, y) {
        wasm.wasmworld_add_obstacle(this.__wbg_ptr, x, y);
    }
    /**
     * Add a predator at position
     * @param {number} x
     * @param {number} y
     */
    add_predator(x, y) {
        wasm.wasmworld_add_predator(this.__wbg_ptr, x, y);
    }
    /**
     * Get bug render data as flat Float32Array
     * Format: [x, y, hue, size, ...]
     * 4 floats per bug
     * @returns {Float32Array}
     */
    get_bug_data() {
        const ret = wasm.wasmworld_get_bug_data(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Get boid render data as flat Float32Array
     * Format: [x, y, vx, vy, hue, energy, max_energy, size_mult, mutations, fear, state, ...]
     * 11 floats per boid
     * @returns {Float32Array}
     */
    get_boid_data() {
        const ret = wasm.wasmworld_get_boid_data(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Get food source data [x, y, amount, ...]
     * @returns {Float32Array}
     */
    get_food_data() {
        const ret = wasm.wasmworld_get_food_data(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Remove obstacle near position (within radius 20)
     * @param {number} x
     * @param {number} y
     */
    remove_obstacle(x, y) {
        wasm.wasmworld_remove_obstacle(this.__wbg_ptr, x, y);
    }
    /**
     * Get obstacle positions as flat array [x, y, x, y, ...]
     * @returns {Float32Array}
     */
    get_obstacle_data() {
        const ret = wasm.wasmworld_get_obstacle_data(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Get predator render data as flat Float32Array
     * Format: [x, y, vx, vy, energy, is_leader, generation, ...]
     * 7 floats per predator
     * @returns {Float32Array}
     */
    get_predator_data() {
        const ret = wasm.wasmworld_get_predator_data(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Create a new simulation world
     * @param {number} width
     * @param {number} height
     * @param {number} start_boids
     */
    constructor(width, height, start_boids) {
        const ret = wasm.wasmworld_new(width, height, start_boids);
        this.__wbg_ptr = ret >>> 0;
        WasmWorldFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Advance simulation by one tick
     * cursor_mode: 0 = none, 1 = attract, 2 = repel
     * @param {number} cursor_x
     * @param {number} cursor_y
     * @param {number} cursor_mode
     * @param {number} cursor_strength
     */
    tick(cursor_x, cursor_y, cursor_mode, cursor_strength) {
        wasm.wasmworld_tick(this.__wbg_ptr, cursor_x, cursor_y, cursor_mode, cursor_strength);
    }
    /**
     * Reset the world
     * @param {number} width
     * @param {number} height
     * @param {number} start_boids
     */
    reset(width, height, start_boids) {
        wasm.wasmworld_reset(this.__wbg_ptr, width, height, start_boids);
    }
    /**
     * Get current width
     * @returns {number}
     */
    width() {
        const ret = wasm.wasmworld_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get current height
     * @returns {number}
     */
    height() {
        const ret = wasm.wasmworld_height(this.__wbg_ptr);
        return ret;
    }
    /**
     * Add a boid at position
     * @param {number} x
     * @param {number} y
     * @param {number} species
     */
    add_boid(x, y, species) {
        wasm.wasmworld_add_boid(this.__wbg_ptr, x, y, species);
    }
    /**
     * Add a food source at position
     * @param {number} x
     * @param {number} y
     */
    add_food(x, y) {
        wasm.wasmworld_add_food(this.__wbg_ptr, x, y);
    }
    /**
     * Add multiple boids at position
     * @param {number} x
     * @param {number} y
     * @param {number} count
     */
    add_boids(x, y, count) {
        wasm.wasmworld_add_boids(this.__wbg_ptr, x, y, count);
    }
    /**
     * Get simulation statistics
     * Returns: [boid_count, predator_count, bug_count, day_phase]
     * @returns {Float32Array}
     */
    get_stats() {
        const ret = wasm.wasmworld_get_stats(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}
if (Symbol.dispose) WasmWorld.prototype[Symbol.dispose] = WasmWorld.prototype.free;

export function init_panic_hook() {
    wasm.init_panic_hook();
}

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('boids_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
