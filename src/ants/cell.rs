
use bevy::prelude::*;

use super::params::{FOOD_COLOR, BOARD_COLOR};

#[derive(Component, Clone)]
pub struct Cell {
    pub food: bool
}

impl Cell {
    pub fn new(food: bool) -> Self {
        Cell { food }
    }

    pub fn leave_food(&mut self) -> Option<()> {
        match self.food {
            true => None,
            false => {
                self.food = true;
                Some(())
            }
        }
    }

    pub fn pickup_food(&mut self) -> Option<()> {
        match self.food {
            false => None,
            true => {
                self.food = false;
                Some(())
            }
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(false)
    }
}

pub fn draw_food(mut query: Query<(&mut Cell, &mut Sprite), Changed<Cell>>) {
    for (cell, mut sprite) in query.iter_mut() {
        if cell.food {
            sprite.color = FOOD_COLOR;
        } else {
            sprite.color = BOARD_COLOR;
        }
    }
}
