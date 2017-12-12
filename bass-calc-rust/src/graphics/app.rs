
use conrod;
use conrod::{Ui, UiCell};
use std::f64;
use graphics::{App, AppInterface};

pub struct BassCalcApp {
    ids: Option<Ids>,
}

widget_ids! {
    pub struct Ids {
        root,
        body,
        param_column,
        param_tabs,
        tabs_scrollbar,
        tab_driver,
        tab_driver_label,
        tab_driver_list,
        tab_graph,
        tab_graph_label,
        graph_column,
        graph_grid,
        graph,
    }
}

pub fn make_app() -> App<BassCalcApp> {
    let app_data = BassCalcApp::new();
    App::new("Bass Calc", (1200, 600), app_data)
}

impl BassCalcApp {

    pub fn new() -> BassCalcApp {
        BassCalcApp {
            ids: None,
        }
    }
}

impl AppInterface for BassCalcApp {
    
    fn initialize(&mut self, ui: &mut Ui) {
        self.ids = Some(Ids::new(ui.widget_id_generator()));
    }

    fn draw(&mut self, ui: &mut UiCell, size: (u32, u32)) {

        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
        use conrod::widget::{Canvas, List, Scrollbar, Tabs, Text};

        let ref ids = self.ids.as_mut().unwrap();

        let (W, H) = size;

        let width = W as f64;
        let param_w = width / 4.0;
        let height = H as f64;

        // Construct our main `Canvas` tree.
        Canvas::new().flow_down(&[
            (ids.body, Canvas::new()),
        ]).set(ids.root, ui);

        Canvas::new().w(param_w).h_of(ids.body).pad(20.0)
            .color(color::LIGHT_ORANGE)
            .mid_left_of(ids.body)
            .set(ids.param_column, ui);

        Canvas::new().w(width - param_w).h_of(ids.body).pad(20.0)
            .right_from(ids.param_column, 0.0)
            .color(color::DARK_CHARCOAL)
            .set(ids.graph_column, ui);

        Tabs::new(&[(ids.tab_driver, "Driver"), (ids.tab_graph, "Graph")])
            .wh_of(ids.param_column)
            .color(color::BLUE)
            .label_color(color::WHITE)
            .middle_of(ids.param_column)
            .set(ids.param_tabs, ui);
        // A scrollbar for the tabs.
        Scrollbar::y_axis(ids.param_tabs).auto_hide(true).set(ids.tabs_scrollbar, ui);

        fn text (text: &str, size: u32) -> Text { Text::new(text).color(color::WHITE).font_size(size) }

        let mut list = vec![true; 4];
        let (mut items, scrollbar) = List::flow_down(list.len())
                .item_size(50.0)
                .scrollbar_on_top()
                .middle_of(ids.tab_driver)
                .wh_of(ids.tab_driver)
                .set(ids.tab_driver_list, ui);

        while let Some(item) = items.next(ui) {
            let i = item.i;
            let title_text = format!("item {}: {}", i, list[i]);
            let title = text(&title_text, 18);
            item.set(title, ui);
        }

        text("Graph", 36).middle_of(ids.tab_graph).set(ids.tab_graph_label, ui);

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
}
