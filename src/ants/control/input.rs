
use bevy::prelude::*;

use super::simulation_state::SimulationState;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<SimulationState>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match *state {
            SimulationState::INITIALIZED => *state = SimulationState::SIMULATING,
            SimulationState::SIMULATING => *state = SimulationState::PAUSED,
            SimulationState::PAUSED => *state = SimulationState::SIMULATING,
            _ => { },
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        match *state {
            _ => *state = SimulationState::FINISHING,
        }
    }
}
