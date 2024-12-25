use std::sync::Mutex;

use crate::utils::get_shape_by_id;
use crate::GameField;

pub trait Block<'a> {
    const ID: u8;
    const FIELD: Mutex<Option<&'a GameField>>;
    fn new(field: &'a GameField, x: i32, y: i32) -> Self;
    fn rotate_clockwise(&mut self);
    fn rotate_counter_clockwise(&mut self);
    fn rotate_180(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn test_collision(&self, dx: i32, dy: i32) -> bool;
    fn get_position(&self) -> (i32, i32);
    fn get_shapes(&self) -> Vec<Vec<Vec<u8>>> {
        get_shape_by_id(Self::ID)
    }
}
