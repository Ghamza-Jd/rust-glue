use wasm_bindgen::prelude::*;
use glue_core::gcd;

#[wasm_bindgen]
pub fn rust_gcd(n: u64, m: u64) -> u64 {
    return gcd(n, m);
}