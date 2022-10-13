use std::time::Duration;

use bevy::{prelude::*};

use crate::ants::{ant, board, cell, input, item, metrics, params, simulation_state, time};

pub struct AntPlugin {
    pub n_ants: i32,
    pub n_food: i32,
    pub max_iterations: i64,
    pub iterations_per_frame: i64,
    pub board_size: i32,
    pub ant_timer: f32,
    pub radius: u8,
    pub alpha: f64,
}

impl Default for AntPlugin {
    fn default() -> Self {
        Self {
            n_ants: 50,
            n_food: 1000,
            max_iterations: 1_000_000,
            iterations_per_frame: 100_000,
            board_size: 70,
            ant_timer: 0.000001,
            radius: 1,
            alpha: 1.0,
        }
    }
}

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(params::Params::new(
            self.n_ants,
            self.n_food,
            self.max_iterations,
            self.iterations_per_frame,
            self.radius,
            self.alpha
        ))
        .init_resource::<board::Board>()
        .insert_resource(simulation_state::SimulationState::INITIALIZED)
        .insert_resource(time::Stopwatch { total: Duration::from_secs(0) })
        .insert_resource(board::Board::new(self.board_size))
        .insert_resource(ant::AntTimer(Timer::from_seconds(self.ant_timer, true)))
        .add_startup_system_to_stage(StartupStage::PreStartup, board::setup_board)
        .add_startup_system(ant::setup_ants)
        .add_system(ant::move_ant)
        .add_system(cell::draw_food)
        .add_system(ant::draw_ant)
        .add_system(input::keyboard_input);
    }
}
