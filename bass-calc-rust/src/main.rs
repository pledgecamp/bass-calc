#![feature(non_ascii_idents)]
#![feature(exclusive_range_pattern)]
#![allow(non_snake_case)]
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate find_folder;
extern crate csv;
extern crate num;
extern crate num_complex;
extern crate uom;

mod parameters;
mod functions;

mod graphics;
use graphics::*;

fn main() {
    let P = parameters::file_defaults();
    
    let mut app = app::make_app(P);
    app.run();
}