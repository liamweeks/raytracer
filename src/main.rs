mod ppm;
mod vec3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod camera;
use crate::ppm::Ppm;
use std::env;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    Ppm::render("output");
}

