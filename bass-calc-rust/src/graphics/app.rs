
use conrod;
use std::f64;

widget_ids! {
    pub struct Ids {
        root,
        body,
        param_column,
        param_tabs,
        tabs_scrollbar,
        tab_driver,
        tab_passive,
        tab_enclosure,
        tab_constant,
        driver_label,
        passive_label,
        enclosure_label,
        constant_label,
        graph_column,
        graph_grid,
        graph,
    }
}

// Draw the Ui.
pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, size: (u32, u32)) {
    use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
    use conrod::widget::{Canvas, Scrollbar, Text};

    let width = size.0 as f64;
    let param_w = width / 3.0;
    let height = size.1 as f64;

    // Construct our main `Canvas` tree.
    Canvas::new().flow_down(&[
        (ids.body, Canvas::new()),
    ]).set(ids.root, ui);

    Canvas::new().w(param_w).h_of(ids.body).pad(20.0)
        .color(color::LIGHT_ORANGE)
        .mid_left_of(ids.body)
        .set(ids.param_column, ui);

    Canvas::new().w(2.0 * param_w).h_of(ids.body).pad(20.0)
        .right_from(ids.param_column, 0.0)
        .color(color::DARK_CHARCOAL)
        .set(ids.graph_column, ui);

    widget::Tabs::new(&[(ids.tab_driver, "Driver"), (ids.tab_passive, "Passive"),
                        (ids.tab_enclosure, "Enclosure"), (ids.tab_constant, "Constants")])
        .wh_of(ids.param_column)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .middle_of(ids.param_column)
        .set(ids.param_tabs, ui);
    // A scrollbar for the tabs.
    Scrollbar::y_axis(ids.param_tabs).auto_hide(true).set(ids.tabs_scrollbar, ui);

    fn text (text: Text) -> Text { text.color(color::WHITE).font_size(36) }
    text(Text::new("Driver")).middle_of(ids.tab_driver).set(ids.driver_label, ui);
    text(Text::new("Enclosure")).middle_of(ids.tab_enclosure).set(ids.enclosure_label, ui);
    text(Text::new("Passive")).middle_of(ids.tab_passive).set(ids.passive_label, ui);
    text(Text::new("Constants")).middle_of(ids.tab_constant).set(ids.constant_label, ui);

    let min_x = 0.0;
    let max_x = f64::consts::PI * 2.0;
    let min_y = -1.0;
    let max_y = 1.0;

    let quarter_lines = widget::grid::Lines::step(0.5_f64).thickness(2.0);
    let sixteenth_lines = widget::grid::Lines::step(0.125_f64).thickness(1.0);
    let lines = &[
        quarter_lines.x(),
        quarter_lines.y(),
        sixteenth_lines.x(),
        sixteenth_lines.y(),
    ];

    widget::Grid::new(min_x, max_x, min_y, max_y, lines.iter().cloned())
        .color(color::rgb(0.1, 0.12, 0.15))
        .wh_of(ids.graph_column)
        .middle_of(ids.graph_column)
        .set(ids.graph_grid, ui);
    widget::PlotPath::new(min_x, max_x, min_y, max_y, f64::sin)
        .color(color::LIGHT_BLUE)
        .thickness(2.0)
        .wh_of(ids.graph_column)
        .middle_of(ids.graph_column)
        .set(ids.graph, ui);
}