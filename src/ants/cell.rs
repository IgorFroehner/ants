
use bevy::prelude::*;

use super::{params::BOARD_COLOR, item::Item};

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

pub fn draw_food(mut query: Query<(&mut Cell, &mut Sprite), Changed<Cell>>, item_query: Query<&Item>) {
    for (cell, mut sprite) in query.iter_mut() {
        match cell.item {
            Some(item) => {
                let item = item_query.get(item).unwrap();

                sprite.color = item.color;
            }
            None => sprite.color = BOARD_COLOR
        }
    }
}
