
use bevy::prelude::*;

use super::{board::Board, params::{Params, ITEM_COLOR}, cell::Cell};

#[derive(Component)]
pub struct Item {
    pub label: String,
    pub color: Color,
    pub data: Vec<f64>,
}

impl Item {
    pub fn difference(&self, other: &Item) -> f64 {
        let mut diff = 0.0f64;

        if self.data.len() != other.data.len() {
            panic!("Tried to compare different data dimensions");
        }
        for i in 0..self.data.len() {
            diff += (self.data[i] - other.data[i]).powi(2) ;
        }
        diff.sqrt()
    }
}

pub fn setup_item(
    mut commands: Commands,
    board: ResMut<Board>,
    params: Res<Params>,
    mut query: Query<&mut Cell>,
) {
    if board.size * board.size < params.food {
        panic!("There are more food then cells in the board.");
    }

    let mut cont = 0;
    while cont < params.food {
        let (x, y) = board.rand_position();

        let mut cell = query.get_mut(board.get_cell_entity(x, y)).unwrap();

        if cell.item.is_none() {
            let entity = commands
                .spawn()
                .insert(Item {
                    label: "dead_ant".to_string(),
                    color: ITEM_COLOR,
                    data: vec![0.0] 
                })
                .id();

            cell.item = Some(entity);
            cont += 1;
        }
    }
}
