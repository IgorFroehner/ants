use bevy::prelude::*;

use super::*;

pub struct AntPlugin {
    pub n_ants: i32,
    pub n_food: i32,
    pub board_size: i32,
    pub ant_timer: f32,
}

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(params::Params::new(self.n_ants, self.n_food))
            .init_resource::<board::Board>()
            .insert_resource(board::Board::new(self.board_size))
            .insert_resource(ant::AntTimer(Timer::from_seconds(self.ant_timer, true)))
            .add_startup_system_to_stage(StartupStage::PreStartup, board::setup_board)
            .add_startup_system(ant::setup_ants)
            .add_startup_system(board::setup_food)
            .add_system(ant::move_ant)
            .add_system(cell::draw_food)
            .add_system(ant::draw_ant);
    }
}
