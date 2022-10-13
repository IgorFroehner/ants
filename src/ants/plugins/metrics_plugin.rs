
use bevy::{prelude::*};

use crate::ants::{metrics, time};

pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(metrics::calculate_metrics)
            .add_system(time::measuere_time);
    }
}
