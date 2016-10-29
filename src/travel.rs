//travel.rs
use std::f64;

pub fn hello_from_travel(dest: &City) {
    // test function to check that travel module can be called from main
    println!("{} is a nice destination.", dest.name);
}

/// convert degrees to radians
fn degrees_to_rad(angle: f64) -> f64 {
    (angle * f64::consts::PI)/180.0f64
}

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub lon: f64,
    pub lat: f64,
}

impl City {
    /// compute the great circle distance between two Cities in km
    pub fn distance(&self, city: &City) -> f64 {
        // convert lon lat from degrees to radians
        let lat0 = degrees_to_rad(self.lat);
        let lon0 = degrees_to_rad(self.lon);
        let lat1 = degrees_to_rad(city.lat);
        let lon1 = degrees_to_rad(city.lon);
        // compute the angle variation between the two ciies
        let dx = lon1.cos() * lat1.cos() - lon0.cos() * lat0.cos();
        let dy = lon1.cos() * lat1.sin() - lon0.cos() * lat0.sin();
        let dz = lat1.sin() - lat0.sin();
        let c = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
        let dsigma = 2.0 * (c/2.0).asin();
        // Earth's radius
        const RADIUS: f64 = 6378f64;
        // return distance in km
        RADIUS * dsigma  
    }
}