/* tslint:disable */
/* eslint-disable */

/**
 * 浏览器侧通过 wasm-bindgen 调用的薄封装。
 */
export class WasmEmulator {
    free(): void;
    [Symbol.dispose](): void;
    load_rom(rom: Uint8Array): void;
    constructor();
    rgbaLen(): number;
    /**
     * 指向最近一次 `sync_rgba` 写入的 RGBA 数据（位于 wasm 线性内存中）。
     */
    rgbaPtr(): number;
    /**
     * `key` 为 0..=15，对应 CHIP-8 十六键。
     */
    set_key(key: number, pressed: boolean): void;
    /**
     * 将当前显示写入 RGBA 缓冲。返回与 `rgba_ptr` 对应的字节长度。
     */
    sync_rgba(): number;
    tick(): void;
    tick_many(count: number): void;
    tick_timers(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmemulator_free: (a: number, b: number) => void;
    readonly wasmemulator_load_rom: (a: number, b: number, c: number) => [number, number];
    readonly wasmemulator_new: () => number;
    readonly wasmemulator_rgbaLen: (a: number) => number;
    readonly wasmemulator_rgbaPtr: (a: number) => number;
    readonly wasmemulator_set_key: (a: number, b: number, c: number) => void;
    readonly wasmemulator_sync_rgba: (a: number) => number;
    readonly wasmemulator_tick: (a: number) => void;
    readonly wasmemulator_tick_many: (a: number, b: number) => void;
    readonly wasmemulator_tick_timers: (a: number) => void;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
