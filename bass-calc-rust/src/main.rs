#![feature(non_ascii_idents)]
#![allow(non_snake_case)]
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate find_folder;
extern crate dimensioned as dim;

mod parameters;
use parameters::*;

#[cfg(all(feature="winit", feature="glium"))] mod graphics;
use graphics::*;

#[cfg(all(feature="winit", feature="glium"))]
fn main() {
    let P = default_parameters();

    draw_loop();
}