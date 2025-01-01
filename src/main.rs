use puyopuyo_tetris::utils;

use bevy::prelude::*;
fn main() {
    App::new().add_systems(Startup, startup).run();
}
fn startup() {
    println!("Hello, Bevy!");
}
