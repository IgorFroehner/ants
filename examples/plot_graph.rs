
use ants::ants::ant::Ant;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;

fn main() {
    plot_prob_func();
}

fn plot_prob_func() {

    let prob =
        Plot::from_function(|x| Ant::prob(x), 0., 1.).line_style(
            LineStyle::new()
                .colour("#35C788")
        );

    let one_minus_prob =
        Plot::from_function(|x| (1.0 - Ant::prob(x)), 0., 1.).line_style(LineStyle::new().colour("burlywood"));

    let v = ContinuousView::new()
        .add(one_minus_prob)
        .add(prob)
        .x_range(0.0, 1.0)
        .y_range(0.0, 1.0)
        .x_label("Score")
        .y_label("Probabilidade");

    Page::single(&v).save("function.svg").expect("saving svg");
}