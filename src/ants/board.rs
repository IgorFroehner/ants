use std::{convert::TryInto};

use bevy::prelude::*;
use rand::{distributions::Uniform, prelude::{Distribution, SliceRandom}, thread_rng};

use crate::MOVE_VEC;

use super::cell::Cell;

pub struct Board {
    pub size: i32,
    pub cells: Vec<Vec<Entity>>,
    uniform: Uniform<i32>,
}

impl Board {
    pub fn new(size: i32) -> Self {
        let size_usize = size.try_into().unwrap();
        Self {
            size,
            cells: vec![vec![Entity::from_raw(0); size_usize]; size_usize],
            uniform: Uniform::from(0..size),
        }
    }

    pub fn set_cell_entity(&mut self, x: i32, y: i32, entity: Entity) {
        let x_usize: usize = x.try_into().unwrap();
        let y_usize: usize = y.try_into().unwrap();
        self.cells[x_usize][y_usize] = entity;
    }

    pub fn get_cell_entity(&self, x: i32, y: i32) -> Entity {
        let x_usize: usize = x.try_into().unwrap();
        let y_usize: usize = y.try_into().unwrap();

        self.cells[x_usize][y_usize]
    }

    pub fn rand_position(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        (
            self.uniform.sample(&mut rng),
            self.uniform.sample(&mut rng),
        )
    }
    
    pub fn cell_size(&self, window: &Window) -> f32 {
        window.width() / self.size as f32
    }
    
    pub fn first_position(&self, window: &Window) -> f32 {
        (-window.width() / 2.0) + self.cell_size(window) / 2.0
    }

    pub fn rand_move(&self, x: i32, y: i32) -> (i32, i32) {
        let mut rng = thread_rng();
        let new_move = MOVE_VEC.choose(&mut rng).unwrap();
        let new_x = (self.size + (x + new_move.0)) % self.size;
        let new_y = (self.size + (y + new_move.1)) % self.size;
    
        (new_x, new_y)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(50)
    }
}

pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<Board>) {
    let window = windows.primary();

    let cell_size: f32 = board.cell_size(window);
    let first_position = board.first_position(window);

    for y in 0..board.size {
        for x in 0..board.size {
            let xcell_position: f32 = first_position + (x as f32) * cell_size;
            let ycell_position: f32 = first_position + (y as f32) * cell_size;

            let entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(xcell_position, ycell_position, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: cell_size - 1.0,
                            y: cell_size - 1.0,
                        }),
                        ..default()
                    },
                    ..default()
                })
                .insert(Cell::default())
                .id();

            board.set_cell_entity(x, y, entity);
        }
    }
}
