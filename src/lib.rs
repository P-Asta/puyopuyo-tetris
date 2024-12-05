mod contracts;
mod tetris;
mod utils;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct GameField {
    blocks: Vec<(u8, u8)>,
    size: (usize, usize),
}

#[wasm_bindgen]
impl GameField {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            size: (0, 0),
        }
    }
}
