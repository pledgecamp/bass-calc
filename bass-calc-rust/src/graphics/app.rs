
use conrod::{Ui, UiCell};
use std::f64;
use graphics::{App, AppInterface, BassGraph};
use functions::Radiator;
use parameters::{Param, Parameters};

use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
use conrod::color::rgb;
use conrod::widget::{id, Id, Canvas, Slider, Rectangle, Scrollbar, Tabs, Text, TextEdit};

pub struct BassCalcApp {
    ids: Option<Ids>,
    title_ids: Vec<[Id; 3]>,
    param_ids: Vec<Vec<[Id; 6]>>,
    params: Parameters,
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
        tab_driver_list_top,
        tab_graph,
        tab_graph_label,
        graph_column,
        graph_grid,
        graph,
    }
}

pub fn make_app(params: Parameters) -> App<BassCalcApp> {
    let app_data = BassCalcApp::new(params);
    App::new("Bass Calc", (1200, 600), app_data)
}

impl BassCalcApp {

    pub fn new(params: Parameters) -> BassCalcApp {
        BassCalcApp {
            ids: None,
            params: params,
            title_ids: vec![],
            param_ids: vec![],
        }
    }

    fn draw_list_title(&self, title: &str, ui: &mut UiCell, ids_index: usize, list_id: Id,
                        prev_id: Id, w: f64, h: f64) -> Id {

        let ids = self.title_ids[ids_index];
        let canvas_id = ids[0];
        let title_id = ids[1];
        let line_id = ids[2];

        Canvas::new().align_middle_x_of(list_id).down_from(prev_id, 0.0).w_of(list_id).h(h)
            .color(color::BLACK).set(canvas_id, ui);

        let _ = text(title, 22)
            .middle_of(canvas_id)
            .set(title_id, ui);

        Rectangle::fill([w - 2.0, 1.0])
            .mid_bottom_of(canvas_id)
            .color(rgb(0.5, 0.5, 0.5))
            .set(line_id, ui);

        canvas_id
    }

    fn draw_list_param(&self, ui: &mut UiCell, ids_index: usize, param_index: usize, param: &Param,
                        list_id: Id, prev_id: Id, w: f64, h: f64) -> Id {

        let name_w = w * 0.17;
        let slider_w = w * 0.28;
        let entry_w = w * 0.25;

        let ids = &self.param_ids[ids_index][param_index];
        let canvas_id = ids[0];
        let line_id = ids[1];
        let name_id = ids[2];
        let slider_id = ids[3];
        let entry_id = ids[4];
        let unit_id = ids[5];
        
        Canvas::new().align_middle_x_of(list_id).down_from(prev_id, 0.0).w_of(list_id).h(h)
            .color(color::DARK_CHARCOAL).set(canvas_id, ui);

        Rectangle::fill([w, 1.0])
            .mid_bottom_of(canvas_id)
            .color(rgb(0.4, 0.4, 0.4))
            .set(line_id, ui);

        let name = format!("{}  ", param.name);
        text(&name, 14).right_justify().mid_left_of(canvas_id).w(name_w).set(name_id, ui);

        let p_val = param.to_percent();
        for value in Slider::new(p_val, 0.0, 1.0)
            .color(color::LIGHT_BLUE)
            .w_h(slider_w, 20.0)
            .align_middle_y_of(canvas_id)
            .right_from(name_id, 0.0)
            .set(slider_id, ui)
        {
            
            param.set_percent(value);
            println!("{} value: {}", param.name, value)
        }

        for _edit in TextEdit::new(&format!("{:.*}", param.precision(), param.v()))
            .color(color::WHITE)
            .w(entry_w)
            .right_from(slider_id, 4.0)
            .center_justify()
            .restrict_to_height(true)
            .set(entry_id, ui)
        {
            
        }

        text(&param.unit, 12).right_from(entry_id, 4.0).set(unit_id, ui);
        canvas_id
    }

    fn draw_list_params(&self, ui: &mut UiCell, ids_index: usize, params: &[Param], list_id: Id,
                        prev_id: Id, w: f64, h: f64) -> Id {

        let mut id = prev_id;
        let mut i = 0;
        for param in params {
            id = self.draw_list_param(ui, ids_index, i, param, list_id, id, w, h);
            i += 1;
        }

        id
    }

    fn draw_params(&self, ui: &mut UiCell, w: f64) {
        let ref ids = self.ids.as_ref().unwrap();

        let driver = &self.params.driver;
        let passive = &self.params.passive;
        let enclosure = &self.params.enclosure;
        let constants = &self.params.constant;

        let h = 38.0;
        
        let list_id = ids.tab_driver_list;
        Canvas::new().color(color::BLACK).scroll_kids_vertically()
            .middle_of(ids.tab_driver)
            .wh_of(ids.tab_driver)
            .set(list_id, ui);

        Scrollbar::y_axis(list_id).auto_hide(false).set(ids.tabs_scrollbar, ui);

        Canvas::new().w_h(0.0, 0.0).mid_top_of(list_id).set(ids.tab_driver_list_top, ui);
        let mut prev_id = ids.tab_driver_list_top;

        prev_id = self.draw_list_title("Driver", ui, 0, list_id, prev_id, w, h);
        prev_id = self.draw_list_params(ui, 0, driver, list_id, prev_id, w, h);

        prev_id = self.draw_list_title("Passive", ui, 1, list_id, prev_id, w, h);
        prev_id = self.draw_list_params(ui, 1, passive, list_id, prev_id, w, h);

        prev_id = self.draw_list_title("Enclosure", ui, 2, list_id, prev_id, w, h);
        prev_id = self.draw_list_params(ui, 2, enclosure, list_id, prev_id, w, h);

        prev_id = self.draw_list_title("Constants", ui, 3, list_id, prev_id, w, h);
        self.draw_list_params(ui, 3, constants, list_id, prev_id, w, h);
    }

}

fn text(text: &str, size: u32) -> Text {
    Text::new(text).color(color::WHITE).font_size(size)
}

fn init_param_ids(id_gen: &mut id::Generator, params: &[Param]) -> Vec<[Id; 6]> {
    let mut ids: Vec<[Id; 6]> = vec![];
    for _ in params.iter() {
        ids.push([id_gen.next(), id_gen.next(), id_gen.next(),
                  id_gen.next(), id_gen.next(), id_gen.next()]);
    }
    ids
}

impl AppInterface for BassCalcApp {
    
    fn initialize(&mut self, ui: &mut Ui) {
        let mut id_gen = ui.widget_id_generator();
        
        self.title_ids = vec![[id_gen.next(), id_gen.next(), id_gen.next()],
                              [id_gen.next(), id_gen.next(), id_gen.next()],
                              [id_gen.next(), id_gen.next(), id_gen.next()],
                              [id_gen.next(), id_gen.next(), id_gen.next()]];


        self.param_ids.push(init_param_ids(&mut id_gen, &self.params.driver));
        self.param_ids.push(init_param_ids(&mut id_gen, &self.params.passive));
        self.param_ids.push(init_param_ids(&mut id_gen, &self.params.enclosure));
        self.param_ids.push(init_param_ids(&mut id_gen, &self.params.constant));

        self.ids = Some(Ids::new(id_gen));
    }

    fn draw(&mut self, ui: &mut UiCell, size: (u32, u32)) {

        let (W, _) = size;

        let width = W as f64;
        let param_w = width / 4.0;

        {
        let ref ids = self.ids.as_mut().unwrap();

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

        //text("Graph", 36).middle_of(ids.tab_graph).set(ids.tab_graph_label, ui);

        let quarter_lines = widget::grid::Lines::step(0.5_f64).thickness(2.0);
        let sixteenth_lines = widget::grid::Lines::step(0.125_f64).thickness(1.0);
        let lines = &[
            quarter_lines.x(),
            quarter_lines.y(),
            sixteenth_lines.x(),
            sixteenth_lines.y(),
        ];

        let min_freq = 20.0;
        let max_freq = 200.0;
        let step = 0.1;

        widget::Grid::new(min_freq, max_freq, -1., 1., lines.iter().cloned())
            .color(color::rgb(0.1, 0.12, 0.15))
            .wh_of(ids.graph_column)
            .middle_of(ids.graph_column)
            .set(ids.graph_grid, ui);
        BassGraph::new(min_freq, max_freq, step, &self.params, Radiator)
            .color(color::LIGHT_BLUE)
            .thickness(2.0)
            .wh_of(ids.graph_column)
            .middle_of(ids.graph_column)
            .set(ids.graph, ui);
        }

        self.draw_params(ui, param_w);
    }
}
