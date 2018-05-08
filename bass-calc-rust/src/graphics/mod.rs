use conrod;
use find_folder;
use conrod::{Ui, UiCell};
use conrod::{image, glium};
use conrod::backend::winit;
use conrod::backend::glium::glium::Surface;
use conrod::backend::glium::Renderer;
use conrod::glium::{Display, glutin};
use conrod::glium::texture::Texture2d;
use conrod::glium::glutin::{ContextBuilder, WindowBuilder};
use std;

pub mod app;

pub trait AppInterface {
    fn initialize(&mut self, &mut Ui);
    fn draw(&mut self, &mut UiCell, (u32, u32));
}

#[cfg(feature="glium")]
pub struct App<T: AppInterface> {
    events_loop: glutin::EventsLoop,
    image_map: image::Map<Texture2d>,
    renderer: Renderer,
    display: Display,
    ui: Ui,
    app_interface: T,
    window_size: (u32, u32),
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

#[cfg(feature="glium")]
impl<T> App<T> where T: AppInterface {

    pub fn new(title: &str, window_size: (u32, u32), mut interface: T) -> App<T> {

        let (WIDTH, HEIGHT) = window_size;

        // Build the window.
        let events_loop = glutin::EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(WIDTH, HEIGHT);
        let context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = Display::new(window, context, &events_loop).unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let resources = find_folder::Search::KidsThenParents(3, 5).for_folder("resources").unwrap();
        let font_path = resources.join("fonts/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let renderer = Renderer::new(&display).unwrap();

        interface.initialize(&mut ui);

        App {
            events_loop: events_loop,
            image_map: image::Map::new(),
            renderer: renderer,
            display: display,
            ui: ui,
            app_interface: interface,
            window_size: window_size,
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    fn next_event(&mut self) -> Vec<glium::glutin::Event> {
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
        self.events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            self.events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

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

    pub fn run(&mut self) {

        'main: loop {

            let events = self.next_event();

            self.ui_needs_update = false;
            self.last_update = std::time::Instant::now();
            // Handle all events.
            for event in events {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = winit::convert_event(event.clone(), &self.display) {
                    self.ui.handle_event(event);
                    self.needs_update();
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
            self.app_interface.draw(&mut self.ui.set_widgets(), self.window_size);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}