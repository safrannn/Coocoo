/* tslint:disable */
/* eslint-disable */
/**
* @param {number} image_id
* @param {number} new_width
* @param {number} new_height
* @returns {number}
*/
export function resize(image_id: number, new_width: number, new_height: number): number;
/**
* @param {number} image_id
* @param {number} value
* @returns {number}
*/
export function darken(image_id: number, value: number): number;
/**
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} a
* @param {number} width
* @param {number} height
* @returns {number}
*/
export function blank_image(r: number, g: number, b: number, a: number, width: number, height: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function grayscale(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function invert_color(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function flip_horizontal(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function flip_vertical(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_r(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_g(image_id: number): number;
/**
* @param {number} image_id
* @returns {number}
*/
export function mask_channel_b(image_id: number): number;
/**
* @param {string} name
* @param {number} width
* @param {number} height
* @param {Uint8Array} pixels
*/
export function library_add_image(name: string, width: number, height: number, pixels: Uint8Array): void;
/**
* @param {any} export_info
* @returns {any}
*/
export function library_export(export_info: any): any;
/**
*/
export function library_reset(): void;
/**
* @param {string} src
* @param {any} image_names
* @returns {any[]}
*/
export function code_to_wasm(src: string, image_names: any): any[];
/**
*/
export class IMAGE_LIBRARY {
  free(): void;
}
