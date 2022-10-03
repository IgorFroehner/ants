
use std::time::Duration;

use bevy::prelude::*;

use super::simulation_state::SimulationState;

pub struct Stopwatch {
    pub total: Duration,
}

pub fn measuere_time(time: Res<Time>, mut stopwatch: ResMut<Stopwatch>, state: Res<SimulationState>) {
    match *state {
        SimulationState::SIMULATING => {
            stopwatch.total += time.delta();
        },
        _ => { },
    }
}
