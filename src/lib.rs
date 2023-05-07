use wasm_bindgen::prelude::*;
use base64::{Engine as _, engine::general_purpose};


#[wasm_bindgen]
pub fn encode(orig: &str) -> String {
    let bytes = orig.as_bytes();
    let encoded_url = general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    encoded_url.to_owned()
}
#[wasm_bindgen]
pub fn decode(orig: &str) -> String {
    let bytes = orig.as_bytes();
    let decoded_bytes = general_purpose::URL_SAFE_NO_PAD.decode(bytes).unwrap();
    String::from_utf8(decoded_bytes).unwrap()
}

