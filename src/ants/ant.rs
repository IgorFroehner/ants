
use std::f64::consts::E;

use bevy::prelude::*;

use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::ants::board::*;
use crate::ants::cell::*;
use crate::ants::params::*;

pub const ANT_IMAGE: &str = "ant.png";
pub const ANT_LOADED_IMAGE: &str = "loaded_ant.png";

pub struct AntTimer(pub Timer);

#[derive(Component)]
pub struct Ant {
    x: i32,
    y: i32,
    loaded: bool,
}

impl Ant {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            loaded: false,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn prob(x: f64) -> f64 {
        // 1.0 / (1.0 + ((-5.0*E*(x-0.4)).exp()))
        1.0 / (1.0 + (E.powf(-E.powf(3.0)*(x-0.43261))))
    }

    pub fn ant_action(&mut self, cell: &mut Cell, score: f64) {
        let prob = Self::prob(score);

        if self.loaded {
            if thread_rng().gen_bool(prob) && cell.leave_food().is_some() {
                self.loaded = false;
            }
        } else {
            if thread_rng().gen_bool(1.0 - prob) && cell.pickup_food().is_some() {
                self.loaded = true;
            }
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
        *image = if ant.loaded {
            asset_server.load(ANT_LOADED_IMAGE)
        } else {
            asset_server.load(ANT_IMAGE)
        };

        let translation = &mut transform.translation;
        translation.x = first_position + (ant.x as f32) * cell_size;
        translation.y = first_position + (ant.y as f32) * cell_size;
    }
}

fn new_rand_position(x: i32, y: i32, board: &Board) -> Option<(i32, i32)> {
    let mut rng = thread_rng();
    let move_vec = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let new_move = move_vec.choose(&mut rng).unwrap();
    let (new_x, new_y) = (x + new_move.0, y + new_move.1);
    if board.valid_position(new_x, new_y) {
        return Some((new_x, new_y));
    }
    None
}

pub fn move_ant(
    mut query_ants: Query<&mut Ant>,
    mut query_cells: Query<&mut Cell>,
    board: ResMut<Board>,
    time: Res<Time>,
    mut timer: ResMut<AntTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for _ in 0..100000 {
            for mut ant in query_ants.iter_mut() {
                let (new_x, new_y) = match new_rand_position(ant.x, ant.y, &board) {
                    None => continue,
                    Some((x, y)) => (x, y)
                };

                let radius = 1;
                let mut food_amount = 0.0;
                let mut n_cells = 0.0;

                for x in ant.x - radius..(ant.x + radius + 1) {
                    for y in ant.y - radius..(ant.y + radius + 1) {
                        if board.valid_position(x, y) {
                            if x == ant.x && y == ant.y {
                                continue;
                            }
                            let cell = query_cells.get(board.get_cell_entity(x, y)).unwrap();

                            if cell.food {
                                food_amount += 1.0;
                            }
                            n_cells += 1.0;
                        }
                    }
                }

                let mut cell = query_cells
                    .get_mut(board.get_cell_entity(ant.x, ant.y))
                    .expect("Error while retrieving the prev cells");

                let food_score: f64 = food_amount / n_cells;

                ant.ant_action(&mut cell, food_score);

                ant.set_position(new_x, new_y);
            }
        }
    }
}