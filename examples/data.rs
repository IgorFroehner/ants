
use bevy::prelude::*;
use bevy::window::PresentMode;

use ants::ants::{AntPlugin, DataPlugin};

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.4, 0.1);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

fn main() {
    println!("Running main example!");

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ants".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AntPlugin {
            n_ants: 30,
            n_food: 1,
            board_size: 70,
            ant_timer: 0.0000001,
            iterations_per_frame: 1_000_000,
            max_iterations: 10_000_000,
            radius: 5,
            alpha: 10.0
        })
        .add_plugin(DataPlugin {
            data_path: "data/dataset_15.txt".to_string()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(start_camera)
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
