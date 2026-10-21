use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(source: &str) -> String {
    format!("you send {}", source)
}
