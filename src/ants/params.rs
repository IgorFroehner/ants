use bevy::prelude::Color;

pub const BOARD_COLOR: Color = Color::rgb(0.55, 0.40, 0.25);
pub const FOOD_COLOR: Color = Color::rgb(0.45, 0.6, 0.25);

pub struct Params {
    pub n_ants: i32,
    pub food: i32
}

impl Params {
    pub fn new(n_ants: i32, food: i32) -> Self {
        Params { n_ants, food }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params::new(100, 250)
    }
}
