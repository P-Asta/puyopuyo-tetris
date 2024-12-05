use crate::contracts::Block;
use crate::utils::OverFlowOps;
use crate::GameField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I {
    pub position: (i32, i32),
    pub rotate_index: i8,
}

impl Block for I {
    const ID: u8 = 0;
    fn new(x: i32, y: i32) -> Self {
        I {
            position: (x, y),
            rotate_index: 0,
        }
    }

    fn rotate(&mut self, n: u8) {
        self.rotate_index.add(n as i8);
    }

    fn move_left(&mut self) {
        self.position.0 -= 1;
    }

    fn move_right(&mut self) {
        self.position.0 += 1;
    }

    fn move_down(&mut self) {
        self.position.1 += 1;
    }

    fn test_collision(&self, dx: i32, dy: i32, field: &GameField) -> bool {
        false
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}
