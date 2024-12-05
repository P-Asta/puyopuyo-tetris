mod contracts;
mod tetris;
mod utils;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct GameField {
    fields: Vec<Vec<u8>>,
    size: (usize, usize),
}

#[wasm_bindgen]
impl GameField {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            size: (0, 0),
        }
    }
}
