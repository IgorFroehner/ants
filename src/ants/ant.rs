use std::f64::consts::E;

use bevy::prelude::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::ants::board::*;
use crate::ants::cell::*;
use crate::ants::params::*;
use crate::ants::simulation_state::SimulationState;

pub const ANT_IMAGE: &str = "ant.png";
pub const ANT_LOADED_IMAGE: &str = "loaded_ant.png";

pub struct AntTimer(pub Timer);

#[derive(Component)]
pub struct Ant {
    x: i32,
    y: i32,
    item: Option<Entity>,
}

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, item: None }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn prob(x: f64) -> f64 {
        1.0 / (1.0 + (E.powf(-E.powf(3.0) * (x - 0.43261))))
    }

    pub fn ant_action(&mut self, cell: &mut Cell, score: f64, ending: bool) -> bool {
        let prob = Self::prob(score);

        match (&self.item, &cell.item) {
            (Some(_), None) => {
                // Ant with item and cell empty
                if thread_rng().gen_bool(prob) {
                    cell.item = self.item.take();
                }
            }
            (None, Some(_)) => {
                // Ant empty and cell with item
                if !ending && thread_rng().gen_bool(1.0 - prob) {
                    self.item = cell.item.take();
                }
            }
            (_, _) => {}
        }

        match self.item {
            Some(_) => true,
            None => false,
        }
    }
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
            .insert(Ant::new(x, y));

        cont += 1;
    }
}

pub fn draw_ant(
    mut query: Query<(&Ant, &mut Transform, &mut Handle<Image>), Changed<Ant>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
) {
    let window = windows.primary();

    let cell_size: f32 = board.cell_size(window);
    let first_position = board.first_position(window);

    for (ant, mut transform, mut image) in query.iter_mut() {
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

fn new_rand_position(x: i32, y: i32, board: &Board) -> (i32, i32) {
    let mut rng = thread_rng();
    let move_vec = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let new_move = move_vec.choose(&mut rng).unwrap();
    let new_x = (board.size + (x + new_move.0)) % board.size;
    let new_y = (board.size + (y + new_move.1)) % board.size;

    (new_x, new_y)
}

pub fn move_ant(
    mut query_ants: Query<&mut Ant>,
    mut query_cells: Query<&mut Cell>,
    board: ResMut<Board>,
    time: Res<Time>,
    mut timer: ResMut<AntTimer>,
    mut params: ResMut<Params>,
    mut state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::SIMULATING => {
            if timer.0.tick(time.delta()).just_finished() {
                for _ in 0..params.iterations_per_frame {
                    for mut ant in query_ants.iter_mut() {
                        let (new_x, new_y) = new_rand_position(ant.x, ant.y, &board);
        
                        let radius = params.radius as i32;
                        let mut food_amount = 0.0;
                        let mut n_cells = 0.0;
        
                        for x in ant.x - radius..(ant.x + radius + 1) {
                            for y in ant.y - radius..(ant.y + radius + 1) {
                                let x = (board.size + x) % board.size;
                                let y = (board.size + y) % board.size;
        
                                if x == ant.x && y == ant.y {
                                    continue;
                                }
                                let cell = query_cells.get(board.get_cell_entity(x, y)).unwrap();
        
                                if cell.item.is_some() {
                                    food_amount += 1.0;
                                }
                                n_cells += 1.0;
                            }
                        }
        
                        let mut cell = query_cells
                            .get_mut(board.get_cell_entity(ant.x, ant.y))
                            .expect("Error while retrieving the prev cells");
        
                        let food_score: f64 = food_amount / n_cells;
        
                        ant.ant_action(&mut cell, food_score, false);
        
                        ant.set_position(new_x, new_y);
                    }
                }
        
                params.max_iterations -= params.iterations_per_frame;
                println!("{}", params.max_iterations);
                if params.max_iterations <= 0 {
                    *state = SimulationState::FINISHING;
                }
            }
        },
        SimulationState::FINISHING => {
            let mut ant_loaded = true;
            while ant_loaded {
                ant_loaded = false;
                for mut ant in query_ants.iter_mut() {
                    let (new_x, new_y) = new_rand_position(ant.x, ant.y, &board);

                    let radius = params.radius as i32;
                    let mut food_amount = 0.0;
                    let mut n_cells = 0.0;

                    for x in ant.x - radius..(ant.x + radius + 1) {
                        for y in ant.y - radius..(ant.y + radius + 1) {
                            let x = (board.size + x) % board.size;
                            let y = (board.size + y) % board.size;

                            if x == ant.x && y == ant.y {
                                continue;
                            }
                            let cell = query_cells.get(board.get_cell_entity(x, y)).unwrap();

                            if cell.item.is_some() {
                                food_amount += 1.0;
                            }
                            n_cells += 1.0;
                        }
                    }

                    let mut cell = query_cells
                        .get_mut(board.get_cell_entity(ant.x, ant.y))
                        .expect("Error while retrieving the prev cells");

                    let food_score: f64 = food_amount / n_cells;

                    if ant.ant_action(&mut cell, food_score, true) {
                        ant_loaded = true;

                        ant.set_position(new_x, new_y);
                    }
                }
            }

            *state = SimulationState::FINISHED;
        }
        _ => { },
    };
}
