use conrod;
use find_folder;
use conrod::glium;
use conrod::glium::glutin;
use conrod::backend::glium::glium::Surface;
use std;

widget_ids! {
    struct Ids {
        root,
        header,
        title,
        body,
        param_column,
        param_title,
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
    }
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
    use conrod::widget::{Text};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().length(300.0).flow_right(&[
            (ids.param_column, widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
            (ids.graph_column, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
        ])),
    ]).set(ids.root, ui);

    widget::Tabs::new(&[(ids.tab_driver, "Driver"), (ids.tab_passive, "Passive"),
                        (ids.tab_enclosure, "Enclosure"), (ids.tab_constant, "Constants")])
        .wh_of(ids.param_column)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .middle_of(ids.param_column)
        .set(ids.param_tabs, ui);
    // A scrollbar for the tabs.
    widget::Scrollbar::y_axis(ids.param_tabs).auto_hide(true).set(ids.tabs_scrollbar, ui);

    Text::new("Bass Calc")
        .color(color::LIGHT_ORANGE)
        .font_size(48)
        .middle_of(ids.header)
        .set(ids.title, ui);

    Text::new("Parameters")
        .color(color::LIGHT_ORANGE.complement())
        .top_left_of(ids.param_column)
        .set(ids.param_title, ui);

    fn text (text: Text) -> Text { text.color(color::WHITE).font_size(36) }
    text(Text::new("Driver")).middle_of(ids.tab_driver).set(ids.driver_label, ui);
    text(Text::new("Enclosure")).middle_of(ids.tab_enclosure).set(ids.enclosure_label, ui);
    text(Text::new("Passive")).middle_of(ids.tab_passive).set(ids.passive_label, ui);
    text(Text::new("Constants")).middle_of(ids.tab_constant).set(ids.constant_label, ui);
}

#[cfg(feature="glium")]
struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

#[cfg(feature="glium")]
impl EventLoop {

    fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    fn next(&mut self, events_loop: &mut glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}

pub fn draw_loop() {
    use conrod::glium::{Display, glutin};
    use conrod::glium::glutin::{ContextBuilder, WindowBuilder};

    const WIDTH: u32 = 900;
    const HEIGHT: u32 = 600;

    // Build the window.
    let mut events_loop = glutin::EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Bass Calc")
        .with_dimensions(WIDTH, HEIGHT);
    let context = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}