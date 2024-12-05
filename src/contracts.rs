use crate::utils::get_shape_by_id;
use crate::GameField;

pub trait Block {
    const ID: u8;
    // const WALL_KICK: ;
    fn new(x: i32, y: i32) -> Self;
    fn rotate(&mut self, n: u8);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn test_collision(&self, dx: i32, dy: i32, field: &GameField) -> bool;
    fn get_position(&self) -> (i32, i32);
    fn get_shapes(&self) -> Vec<Vec<Vec<u8>>> {
        get_shape_by_id(Self::ID)
    }
}
