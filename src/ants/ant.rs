use std::f64::consts::E;

use bevy::prelude::*;

use rand::thread_rng;
use rand::Rng;

use crate::ants::board::*;
use crate::ants::cell::*;
use crate::ants::params::*;
use crate::ants::simulation_state::SimulationState;
use crate::{ANT_IMAGE, ANT_LOADED_IMAGE};

use super::item::Item;

pub struct AntTimer(pub Timer);

#[derive(Component)]
pub struct Ant {
    x: i32,
    y: i32,
    item: Option<Entity>,
    k1: f64,
    k2: f64
}

impl Ant {
    pub fn new(x: i32, y: i32, k1: f64, k2: f64) -> Self {
        Self { x, y, item: None, k1, k2 }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        // println!("{}, {}", x, y);
        self.x = x;
        self.y = y;
    }

    pub fn prob(&self, x: f64) -> f64 {
        // (x / self.k2 + x).powf(2.0).min(0.0).max(1.0)
        1.0 / (1.0 + (E.powf(-32.0 * (x - 0.35))))
    }

    // pub fn ant_action(&mut self, cell: &mut Cell, score: f64, ending: bool) -> bool {
    //     let prob = Self::prob(score);

    //     match (&self.item, &cell.item) {
    //         (Some(_), None) => {
    //             // Ant with item and cell empty
    //             if thread_rng().gen_bool(prob) {
    //                 cell.item = self.item.take();
    //             }
    //         }
    //         (None, Some(_)) => {
    //             // Ant empty and cell with item
    //             if !ending && thread_rng().gen_bool(1.0 - prob) {
    //                 self.item = cell.item.take();
    //             }
    //         }
    //         (_, _) => {}
    //     }

    //     match self.item {
    //         Some(_) => true,
    //         None => false,
    //     }
    // }
}

pub fn setup_ants(
    mut commands: Commands,
    windows: Res<Windows>,
    board: ResMut<Board>,
    params: Res<Params>,
    asset_server: ResMut<AssetServer>,
) {
    if board.size * board.size < params.n_ants {
        panic!("There are more ants then cells in the board.");
    }

    let window = windows.primary();

    let cell_size: f32 = board.cell_size(window);
    let first_position = board.first_position(window);

    let mut cont = 0;
    while cont < params.n_ants {
        let (x, y) = board.rand_position();

        let x_position = first_position + x as f32 * cell_size;
        let y_position = first_position + y as f32 * cell_size;

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(x_position, y_position, 2.0),
                texture: asset_server.load(ANT_IMAGE),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: cell_size,
                        y: cell_size,
                    }),
                    ..default()
                },
                ..default()
            })
            .insert(Ant::new(x, y, params.k1, params.k2));

        cont += 1;
    }
}

pub fn draw_ant(
    mut query: Query<(&Ant, &mut Transform, &mut Handle<Image>, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
    state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::FINISHING => {
            for (_, _, _, entity) in query.iter() {
                commands.entity(entity).despawn();
            }
        },
        _ => {
            let window = windows.primary();

            let cell_size: f32 = board.cell_size(window);
            let first_position = board.first_position(window);

            for (ant, mut transform, mut image, _) in query.iter_mut() {
                *image = if ant.item.is_some() {
                    asset_server.load(ANT_LOADED_IMAGE)
                } else {
                    asset_server.load(ANT_IMAGE)
                };

                let translation = &mut transform.translation;
                translation.x = first_position + (ant.x as f32) * cell_size;
                translation.y = first_position + (ant.y as f32) * cell_size;
            }
        }
    }
}

pub fn move_ant(
    mut query_ants: Query<&mut Ant>,
    mut query_cells: Query<&mut Cell>,
    query_item: Query<&Item>,
    board: ResMut<Board>,
    time: Res<Time>,
    mut timer: ResMut<AntTimer>,
    mut params: ResMut<Params>,
    mut state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::SIMULATING => {
            if timer.0.tick(time.delta()).just_finished() {
                for _ in  0..params.iterations_per_frame {
                    ant_actions(&mut query_ants, &board, &params, &mut query_cells, &query_item, false);
                }

                params.max_iterations -= params.iterations_per_frame;
                // println!("{}", params.max_iterations);
                if params.max_iterations <= 0 {
                    *state = SimulationState::FINISHING;
                }
            }
        },
        SimulationState::FINISHING => {
            while ant_actions(&mut query_ants, &board, &params, &mut query_cells, &query_item, true) {}
            // let ant_loaded = ;

            // if !ant_loaded {
                *state = SimulationState::FINISHED;
            // }

            println!("Finished");
        },
        _ => { },
    };
}

fn ant_actions(
    query_ants: &mut Query<&mut Ant>, 
    board: &ResMut<Board>, 
    params: &ResMut<Params>, 
    query_cells: &mut Query<&mut Cell>,
    query_item: &Query<&Item>, 
    ending: bool
) -> bool {
    let mut cont_ants_loaded: u32 = 0;
    let alpha = params.alpha;

    for mut ant in query_ants.iter_mut() {
        let current_cell = query_cells
            .get_mut(board.get_cell_entity(ant.x, ant.y))
            .expect("Error while retrieving the prev cells");

        match (ant.item, current_cell.item) {
            (Some(ant_item), None) => { // If the action is to drop
                // println!("ant X cell O");
                let ant_item = query_item.get(ant_item).unwrap();
                let radius = params.radius as i32;
                let mut diff_sum = 0.0;
                let mut n_cells = 0.0;

                for x in ant.x - radius..(ant.x + radius + 1) {
                    for y in ant.y - radius..(ant.y + radius + 1) {
                        let x = (board.size + x) % board.size;
                        let y = (board.size + y) % board.size;

                        if x == ant.x && y == ant.y {
                            continue;
                        }
                        let cell = query_cells.get(board.get_cell_entity(x, y)).unwrap();

                        match cell.item {
                            Some(cell_item_entity) => {
                                let cell_item = query_item.get(cell_item_entity).unwrap();

                                diff_sum += 1.0 - (ant_item.difference(&cell_item) / alpha);
                            },
                            None => {}
                        }

                        n_cells += 1.0;
                    }
                }

                let mut cell = query_cells
                    .get_mut(board.get_cell_entity(ant.x, ant.y))
                    .expect("Error while retrieving the prev cells");

                let position_score: f64 = diff_sum / n_cells;
                let prob = ant.prob(position_score);

                if thread_rng().gen_bool(prob) {
                    cell.item = ant.item.take();
                }

                if ant.item.is_some() {
                    cont_ants_loaded += 1;
                }

                if ending && ant.item.is_some() {
                    let (new_x, new_y) = board.rand_move(ant.x, ant.y);

                    ant.set_position(new_x, new_y);
                }
            }
            (None, Some(cell_item)) => { // If the action is to pick
                // println!("ant X cell O");

                let cell_item = query_item.get(cell_item).unwrap();

                let radius = params.radius as i32;
                let mut diff_sum = 0.0;
                let mut n_cells = 0.0;

                for x in ant.x - radius..(ant.x + radius + 1) {
                    for y in ant.y - radius..(ant.y + radius + 1) {
                        let x = (board.size + x) % board.size;
                        let y = (board.size + y) % board.size;

                        if x == ant.x && y == ant.y {
                            continue;
                        }
                        let cell = query_cells.get(board.get_cell_entity(x, y)).unwrap();

                        match cell.item {
                            Some(other_cell_item) => {
                                let other_cell_item = query_item.get(other_cell_item).unwrap();

                                diff_sum += 1.0 - (cell_item.difference(&other_cell_item) / alpha);
                            },
                            None => {}
                        }

                        n_cells += 1.0;
                    }
                }

                let mut cell = query_cells
                    .get_mut(board.get_cell_entity(ant.x, ant.y))
                    .expect("Error while retrieving the prev cells");

                let position_score: f64 = diff_sum / n_cells;
                let prob = ant.prob(position_score);

                if !ending {
                    if thread_rng().gen_bool(1.0 - prob) {
                        ant.item = cell.item.take();
                    }
                }
            },
            (_, _) => {
                let (new_x, new_y) = board.rand_move(ant.x, ant.y);

                ant.set_position(new_x, new_y);
            },
        }
        if !ending {
            let (new_x, new_y) = board.rand_move(ant.x, ant.y);

            ant.set_position(new_x, new_y);
        }
    }

    cont_ants_loaded > 0
}
