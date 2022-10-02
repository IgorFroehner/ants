
use bevy::prelude::*;
use bevy::window::PresentMode;

use ants::ants::AntPlugin;

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
            n_ants: 100,
            n_food: 4000,
            board_size: 100,
            ant_timer: 0.000001,
            iterations_per_frame: 10_000,
            max_iterations: 1_000_000,
            radius: 1,
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(start_camera)
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
