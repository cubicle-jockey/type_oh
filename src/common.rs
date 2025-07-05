use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}
pub fn random_byte(min_val: u8, max_val: u8) -> u8 {
    // I can't get the `rand` crate to work with wasm, so rolling my own.
    let range = (max_val - min_val + 1) as f64;
    let random_val = (random() * range).floor() as u8;
    min_val + random_val
}

pub fn now() -> u64 {
    // Get the current time in milliseconds since the Unix epoch
    js_sys::Date::now() as u64
}
