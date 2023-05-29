use crate::ray::Ray;
use crate::vec3::*;

use std::fs::File;
use std::io::Write;

pub fn print_colour(colour: Colour) {
    // Write the translated [0, 255] value of each colour component
    let ir = (colour.x() * 255.999) as i32;
    let ig = (colour.y() * 255.999) as i32;
    let ib = (colour.z() * 255.999) as i32;

    println!("{} {} {}", ir, ig, ib);
}

