use bevy::prelude::Color;

pub const BOARD_COLOR: Color = Color::rgb(0.2, 0.5, 0.2);
pub const ITEM_COLOR: Color = Color::rgb(0.8, 0.2, 0.1);

pub struct Params {
    pub n_ants: i32,
    pub food: i32,
    pub max_iterations: i64,
    pub iterations_per_frame: i64,
    pub radius: u8,
}

impl Params {
    pub fn new(n_ants: i32, food: i32, max_iterations: i64, iterations_per_frame: i64, radius: u8) -> Self {
        Params { n_ants, food, max_iterations, iterations_per_frame, radius }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params::new(100, 250, 100_000_000, 10_000, 1)
    }
}
