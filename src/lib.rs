pub mod contracts;
pub mod tetris;
pub mod utils;

extern "C" {
    fn alert(s: &str);
}

pub struct GameField {
    blocks: Vec<(u8, u8)>,
    size: (usize, usize),
}

impl GameField {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            size: (0, 0),
        }
    }
}
