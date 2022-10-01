use bevy::prelude::*;

use crate::ants::{board::Board, cell::Cell};

use super::simulation_state::SimulationState;

pub fn calculate_metrics(
    state: Res<SimulationState>,
    board: Res<Board>,
    query_cells: Query<&mut Cell>,
) {
    match *state {
        SimulationState::ENDED => {
            calculate_biggest_and_number_of_cluster(board, query_cells);
        },
        _ => {},
    }
}

fn calculate_biggest_and_number_of_cluster(board: Res<Board>, query_cells: Query<&mut Cell>) {
    let vis = vec![vec![false; board.size.try_into().unwrap()]; board.size.try_into().unwrap()];


}

