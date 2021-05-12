import * as wasm from './compiler_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

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

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function logError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            let error = (function () {
                try {
                    return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
                } catch(_) {
                    return "<failed to stringify thrown value>";
                }
            }());
            console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
            throw e;
        }
    };
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
/**
* @param {number} image_id
* @param {number} new_width
* @param {number} new_height
* @returns {number}
*/
export function resize(image_id, new_width, new_height) {
    _assertNum(image_id);
    _assertNum(new_width);
    _assertNum(new_height);
    var ret = wasm.resize(image_id, new_width, new_height);
    return ret;
}

/**
* @param {number} image_id
* @param {number} value
* @returns {number}
*/
export function darken(image_id, value) {
    _assertNum(image_id);
    _assertNum(value);
    var ret = wasm.darken(image_id, value);
    return ret;
}

/**
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} a
* @param {number} width
* @param {number} height
* @returns {number}
*/
export function blank_image(r, g, b, a, width, height) {
    _assertNum(r);
    _assertNum(g);
    _assertNum(b);
    _assertNum(a);
    _assertNum(width);
    _assertNum(height);
    var ret = wasm.blank_image(r, g, b, a, width, height);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function grayscale(image_id) {
    _assertNum(image_id);
    var ret = wasm.grayscale(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function invert_color(image_id) {
    _assertNum(image_id);
    var ret = wasm.invert_color(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function flip_horizontal(image_id) {
    _assertNum(image_id);
    var ret = wasm.flip_horizontal(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function flip_vertical(image_id) {
    _assertNum(image_id);
    var ret = wasm.flip_vertical(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_r(image_id) {
    _assertNum(image_id);
    var ret = wasm.mask_channel_r(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_g(image_id) {
    _assertNum(image_id);
    var ret = wasm.mask_channel_g(image_id);
    return ret;
}

/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_b(image_id) {
    _assertNum(image_id);
    var ret = wasm.mask_channel_b(image_id);
    return ret;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {string} name
* @param {number} width
* @param {number} height
* @param {Uint8Array} pixels
*/
export function library_add_image(name, width, height, pixels) {
    var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    _assertNum(width);
    _assertNum(height);
    var ptr1 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    wasm.library_add_image(ptr0, len0, width, height, ptr1, len1);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

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
/**
* @param {any} export_info
* @returns {any}
*/
export function library_export(export_info) {
    try {
        var ret = wasm.library_export(addBorrowedObject(export_info));
        return takeObject(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
*/
export function library_reset() {
    wasm.library_reset();
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}
/**
* @param {string} src
* @param {any} image_names
* @returns {any[]}
*/
export function code_to_wasm(src, image_names) {
    try {
        const retptr = wasm.__wbindgen_export_2.value - 16;
        wasm.__wbindgen_export_2.value = retptr;
        var ptr0 = passStringToWasm0(src, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.code_to_wasm(retptr, ptr0, len0, addBorrowedObject(image_names));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4);
        return v1;
    } finally {
        wasm.__wbindgen_export_2.value += 16;
        heap[stack_pointer++] = undefined;
    }
}

/**
*/
export class IMAGE_LIBRARY {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_image_library_free(ptr);
    }
}

export const __wbg_log_8a9e68e9d577d879 = logError(function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
});

export const __wbindgen_json_parse = function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbindgen_json_serialize = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

