/* tslint:disable */
/* eslint-disable */

export class WasmWorld {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Add an obstacle at position
   */
  add_obstacle(x: number, y: number): void;
  /**
   * Add a predator at position
   */
  add_predator(x: number, y: number): void;
  /**
   * Get bug render data as flat Float32Array
   * Format: [x, y, hue, size, ...]
   * 4 floats per bug
   */
  get_bug_data(): Float32Array;
  /**
   * Get boid render data as flat Float32Array
   * Format: [x, y, vx, vy, hue, energy, max_energy, size_mult, mutations, fear, state, ...]
   * 11 floats per boid
   */
  get_boid_data(): Float32Array;
  /**
   * Get food source data [x, y, amount, ...]
   */
  get_food_data(): Float32Array;
  /**
   * Get obstacle positions as flat array [x, y, x, y, ...]
   */
  get_obstacle_data(): Float32Array;
  /**
   * Get predator render data as flat Float32Array
   * Format: [x, y, vx, vy, energy, is_leader, generation, ...]
   * 7 floats per predator
   */
  get_predator_data(): Float32Array;
  /**
   * Create a new simulation world
   */
  constructor(width: number, height: number, start_boids: number);
  /**
   * Advance simulation by one tick
   * cursor_mode: 0 = none, 1 = attract, 2 = repel
   */
  tick(cursor_x: number, cursor_y: number, cursor_mode: number, cursor_strength: number): void;
  /**
   * Reset the world
   */
  reset(width: number, height: number, start_boids: number): void;
  /**
   * Get current width
   */
  width(): number;
  /**
   * Get current height
   */
  height(): number;
  /**
   * Add a boid at position
   */
  add_boid(x: number, y: number, species: number): void;
  /**
   * Add a food source at position
   */
  add_food(x: number, y: number): void;
  /**
   * Add multiple boids at position
   */
  add_boids(x: number, y: number, count: number): void;
  /**
   * Get simulation statistics
   * Returns: [boid_count, predator_count, bug_count, day_phase]
   */
  get_stats(): Float32Array;
}

export function init_panic_hook(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmworld_free: (a: number, b: number) => void;
  readonly wasmworld_add_boid: (a: number, b: number, c: number, d: number) => void;
  readonly wasmworld_add_boids: (a: number, b: number, c: number, d: number) => void;
  readonly wasmworld_add_food: (a: number, b: number, c: number) => void;
  readonly wasmworld_add_obstacle: (a: number, b: number, c: number) => void;
  readonly wasmworld_add_predator: (a: number, b: number, c: number) => void;
  readonly wasmworld_get_boid_data: (a: number) => [number, number];
  readonly wasmworld_get_bug_data: (a: number) => [number, number];
  readonly wasmworld_get_food_data: (a: number) => [number, number];
  readonly wasmworld_get_obstacle_data: (a: number) => [number, number];
  readonly wasmworld_get_predator_data: (a: number) => [number, number];
  readonly wasmworld_get_stats: (a: number) => [number, number];
  readonly wasmworld_height: (a: number) => number;
  readonly wasmworld_new: (a: number, b: number, c: number) => number;
  readonly wasmworld_reset: (a: number, b: number, c: number, d: number) => void;
  readonly wasmworld_tick: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wasmworld_width: (a: number) => number;
  readonly init_panic_hook: () => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
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
