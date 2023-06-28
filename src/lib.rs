mod utils;
mod bech32str;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn encode(prefix: &str, base64_str: &str) -> JsValue  {

    let encoded_str = &bech32str::encode(
        &base64::decode(base64_str).unwrap(),
        prefix,
        bech32str::Bech32m,
    );

    return serde_wasm_bindgen::to_value(&encoded_str).unwrap();
}


