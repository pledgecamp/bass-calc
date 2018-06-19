#![feature(non_ascii_idents)]
#![feature(exclusive_range_pattern)]
#![allow(non_snake_case)]
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate find_folder;
extern crate csv;
extern crate num;

mod parameters;

#[cfg(all(feature="winit", feature="glium"))] mod graphics;
use graphics::*;

#[cfg(all(feature="winit", feature="glium"))]
fn main() {
    let P = parameters::file_defaults();
    
    let mut app = app::make_app(P);
    app.run();
}