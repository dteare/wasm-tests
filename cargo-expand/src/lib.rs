extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u16, b: u8) ->  u16 {
  a + (b as u16)
}
