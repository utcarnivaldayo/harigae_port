/* tslint:disable */
/* eslint-disable */
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
export function iir_design_pso(numerator_order: number, denominator_order: number, pass_band_edge: number, stop_band_edge: number, desired_group_delay: number, max_ripple: number, division_approximation_band: number, division_transition_band: number, number_of_search_points: number, max_iteration: number, weight: number, c1: number, c2: number, init_scale: number, init_a: number, normalized_angular_frequency: Float64Array, magnitude_response: Float64Array, group_delay: Float64Array, a: Float64Array, b: Float64Array): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly iir_design_pso: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        