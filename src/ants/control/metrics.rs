use bevy::{prelude::*};

use crate::{ants::{board::{Board}, cell::Cell}, MOVE_VEC};

use queues::*;


use super::{simulation_state::SimulationState, time::Stopwatch};

pub fn calculate_metrics(
    mut state: ResMut<SimulationState>,
    board: Res<Board>,
    query_cells: Query<&mut Cell>,
    stopwatch: Res<Stopwatch>,
) {
    match *state {
        SimulationState::FINISHED => {
            calculate_biggest_and_number_of_cluster(board, query_cells);
            
            println!("time: {} seconds", stopwatch.total.as_secs());
            
            *state = SimulationState::ENDED;
        },
        _ => {},
    }
}

fn get_pos(x: i32, y: i32, board_size: i32) -> (i32, i32) {
    (
        (board_size + x) % board_size,
        (board_size + y) % board_size
    )
}

fn calculate_biggest_and_number_of_cluster(board: Res<Board>, query_cells: Query<&mut Cell>) {
    let mut vis = vec![vec![false; board.size.try_into().unwrap()]; board.size.try_into().unwrap()];
    let mut biggest_cluster = -1;
    let mut n_clusters = 0;
    let mut total_items = 0;

    for x in 0..board.size {
        for y in 0..board.size {
            let cell = query_cells.get(board.get_cell_entity(x as i32, y as i32)).unwrap();
            let ux: usize = x.try_into().unwrap();
            let uy: usize = y.try_into().unwrap();

            if cell.item.is_some() && !vis[ux][uy] {
                let mut q: Queue<(i32, i32)> = queue![];
                q.add((x, y)).unwrap();
                vis[ux][uy] = true;
                let mut cont = 0;

                while q.size() != 0 {
                    let (kx, ky) = q.remove().unwrap();
                    // let ukx: usize = kx.try_into().unwrap();
                    // let uky: usize = ky.try_into().unwrap();
                    // vis[ukx][uky] = true;
                    cont += 1;

                    for (move_x, move_y) in &MOVE_VEC {
                        let (adj_x, adj_y) = get_pos(move_x + kx, move_y + ky, board.size);
                        let uadj_x: usize = adj_x.try_into().unwrap();
                        let uadj_y: usize = adj_y.try_into().unwrap();

                        if vis[uadj_x][uadj_y] { continue; }

                        let new_cell = query_cells.get(board.get_cell_entity(adj_x, adj_y)).unwrap();

                        if new_cell.item.is_some() {
                            q.add((adj_x, adj_y)).unwrap();
                            vis[uadj_x][uadj_y] = true;
                        }
                    }
                }

                total_items += cont;
                biggest_cluster = biggest_cluster.max(cont);
                n_clusters += 1;
            }
        }
    }

    println!("total_food: {}", total_items);
    println!("biggest cluster: {}", biggest_cluster);
    println!("n_clusters: {}", n_clusters);
}

// fn bfs(x: i32, y: i32, query_cells: &Query<&mut Cell>) -> i32 {
//     1
// }
