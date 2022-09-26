use bevy::prelude::*;
use bevy::window::PresentMode;

mod ants;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.2, 0.2);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

fn main() {
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
        .add_plugin(ants::ant_plugin::AntPlugin {
            n_ants: 100,
            n_food: 3000,
            board_size: 100,
            ant_timer: 0.0001,
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(start_camera)
        .add_startup_system(plot_prob_func)
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn plot_prob_func() {
    use plotlib::page::Page;
    use plotlib::repr::Plot;
    use plotlib::style::LineStyle;
    use plotlib::view::ContinuousView;

    let f1 =
        Plot::from_function(|x| ants::ant::Ant::prob(x), 0., 1.).line_style(LineStyle::new().colour("burlywood"));

    let v = ContinuousView::new().add(f1);

    Page::single(&v).save("function.svg").expect("saving svg");
}
