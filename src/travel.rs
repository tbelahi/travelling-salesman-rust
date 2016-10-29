//travel.rs
use std::f64;

pub fn hello_from_travel(dest: &str) {
    // test function to check that travel module can be called from main
    println!("{} is a nice destination.", dest);
    let angle = 45f64;
    println!("{}", degrees_to_rad(angle));
}

fn degrees_to_rad(angle: f64) -> f64 {
    (angle * 180.0f64)/f64::consts::PI
}