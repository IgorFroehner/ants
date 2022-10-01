
use bevy::prelude::*;

use super::params::{ITEM_COLOR, BOARD_COLOR};

#[derive(Component, Clone)]
pub struct Cell {
    pub item: Option<Entity>
}

impl Cell {
    pub fn new() -> Self {
        Cell { item: None }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

pub fn draw_food(mut query: Query<(&mut Cell, &mut Sprite), Changed<Cell>>) {
    for (cell, mut sprite) in query.iter_mut() {
        if cell.item.is_some() {
            sprite.color = ITEM_COLOR;
        } else {
            sprite.color = BOARD_COLOR;
        }
    }
}
