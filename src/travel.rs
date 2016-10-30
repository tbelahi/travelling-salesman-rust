//travel.rs

use csv;
use std::f64;
use std::path::Path;

pub fn hello_from_travel(dest: &City) {
    // test function to check that travel module can be called from main
    println!("{} is a nice destination.", dest.name);
}

/// convert degrees to radians
fn degrees_to_rad(angle: f64) -> f64 {
    (angle * f64::consts::PI)/180.0f64
}

#[derive(Debug,RustcDecodable)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
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

/// take whatever can be casted to a Path and read the corresponding csv file.
/// The csv file should be such that
/// 1  ville,latitude,longitude
/// 2  Paris,48.8575,2.3458
/// 3  Lyon,45.7564,4.8333
/// 4  ...
/// The function return a Vec<City>, a list of the cities in the csv file parsed into
/// the structure City.
pub fn load_cities_from_file<P: AsRef<Path>>(path: P) -> Vec<City> {
    let mut rdr = csv::Reader::from_file(path).unwrap();
    let mut cities: Vec<City> = Vec::new();
    for city in rdr.decode() {
        let city: City = city.unwrap();
        cities.push(city);
    }
    cities
}

// TODO: is it good idea to implement type alias Vec<City> et define function to manipulate it
// or should I define a new struct Cities {list: Vec<City>, other info...} ?

#[cfg(test)]
mod tests {
    use travel::*;
    // test material

    #[test]
    fn test_distance() {
        let paris = City { name: "Paris".to_string(), lon: 2.3476, lat: 48.8543 };
        assert!(paris.distance(&paris) == 0f64);
    }

    #[test]
    fn commutative_test() {
        let paris = City { name: "Paris".to_string(), lon: 2.3476, lat: 48.8543 };
        let marseille = City { name: "Marseille".to_string(), lon: 5.3700, lat: 43.2948 };
        assert!(paris.distance(&marseille) == marseille.distance(&paris));
    }
}