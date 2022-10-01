
use bevy::prelude::*;

use ants::ants::ant::Ant;

fn main() {
    App::new()
        .add_startup_system(plot_prob_func)
        .run();
}

fn plot_prob_func() {
    use plotlib::page::Page;
    use plotlib::repr::Plot;
    use plotlib::style::LineStyle;
    use plotlib::view::ContinuousView;

    let f1 =
        Plot::from_function(|x| Ant::prob(x), 0., 1.).line_style(LineStyle::new().colour("burlywood"));

    let v = ContinuousView::new().add(f1);

    Page::single(&v).save("function.svg").expect("saving svg");
}