use crate::utils::get_shape_by_id;
use crate::GameField;
use std::time::{Duration, Instant};

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
trait Timed {
    fn start_timer(&mut self, duration: u64);
    fn stop_timer(&mut self);
    fn reset_timer(&mut self);
    fn is_timer_expired(&self) -> bool;
}
struct Timer {
    start: Option<Instant>,
    duration: Duration,
}

impl Timer {
    fn new() -> Self {
        Timer {
            start: None,
            duration: Duration::new(0, 0),
        }
    }
}

impl Timed for Timer {
    fn start_timer(&mut self, duration: u64) {
        self.start = Some(Instant::now());
        self.duration = Duration::new(duration, 0);
    }

    fn stop_timer(&mut self) {
        self.start = None;
    }

    fn reset_timer(&mut self) {
        self.start = Some(Instant::now());
    }

    fn is_timer_expired(&self) -> bool {
        if let Some(start) = self.start {
            start.elapsed() >= self.duration
        } else {
            false
        }
    }
}
