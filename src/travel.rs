//travel.rs

use csv;
use std::f64;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub fn hello_from_travel(dest: &City) {
    // test function to check that travel module can be called from main
    println!("{} is a nice destination.", dest.name);
}

/// convert degrees to radians
fn degrees_to_rad(angle: f64) -> f64 {
    (angle * f64::consts::PI)/180.0f64
}

#[derive(Debug, RustcDecodable)]
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
        let dx: f64 = lon1.cos() * lat1.cos() - lon0.cos() * lat0.cos();
        let dy: f64 = lon1.cos() * lat1.sin() - lon0.cos() * lat0.sin();
        let dz: f64 = lat1.sin() - lat0.sin();
        let c : f64= (dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)).sqrt();
        let dsigma: f64 = 2.0 * ((c/2.0f64).asin());
        // Earth's radius
        const RADIUS: f64 = 6400f64;
        // return distance in km
        RADIUS * dsigma  
    }
}

/// take whatever can be casted to a Path and read the corresponding csv file.
/// The csv file should be such that:
///
/// 1  ville,latitude,longitude
/// 2  Paris,48.8575,2.3458
/// 3  Lyon,45.7564,4.8333
/// 4  ...
///
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

/// Loads into a Vec<String> a list of cities to visit for the travelling salesman
/// The list of cities should be provided as a column in a text file such as:
/// example.txt
///
/// 1  Paris
/// 2  Toulouse
/// 3  Montpellier
/// 4  ...
///
pub fn load_travel_plan_from_file(path: &str) -> Vec<String> {
    // ouvrir le fichier en mode lecture seule
    let path = Path::new(path);
    let mut file = match File::open(&path) {
        // la méthode "descriptive" io::Error retourne une chaine de charactèère
        // qui décrit l'erreur
        Err(why) => panic!("couldn't open {:?}: {}", path.display(),
                           why.description()),
        Ok(file) => file,
   };

   let mut s = String::new();
   file.read_to_string(&mut s).ok().expect("failed to read travel file");
   let travel_plan: Vec<String> = s.split('\n')
                                   .map(|x| x.trim().to_string())
                                   .collect();
   travel_plan
}

pub fn name_to_city<'a>(name: &str, cities: &'a Vec<City>) -> &'a City {
    let ix = cities.iter().position(|c| c.name == name).unwrap();
    &cities[ix]
}

pub fn cost_of_travel_plan(plan: &Vec<String>, cities: &Vec<City>) -> f64 {
    let villes: Vec<&City> = plan.into_iter().map(|x| name_to_city(x, &cities)).collect();
    println!("{:?}", villes);
    //let cout = zip(villes[0..-1], villes[1..]).fold(0, |&mut acc(x, y| acc +  x.distance(y)).unwrap();
    let len = villes.len();
    let mut cout: f64 = 0f64;
    for i in 0..len-1 {
        println!("la distance entre {} et {} est {} km", 
                villes[i].name, villes[i+1].name, villes[i].distance(&villes[i+1]));
        cout = cout + villes[i].distance(villes[i+1]);
    }
    cout
}

fn shuffle<'a>(plan: &'a Vec<String>, iteration: &i32) -> &'a Vec<String> {
     if iteration % 2 == 0 {
       unimplemented!();
    } else {
       unimplemented!();
    }
} 

fn accept(new_plan: &Vec<String>, old_plan: &Vec<String>, temperature: &f64, cities: &Vec<City>) -> bool {
    let new_cost = cost_of_travel_plan(&new_plan, &cities);
    let old_cost = cost_of_travel_plan(&old_plan, &cities);
    unimplemented!();
}

pub fn optimize_travel<'a>(plan: &'a Vec<String>, cities: &Vec<City>, init_temp: f64, cooling_speed: f64, max_iter: i32) -> (&'a Vec<String>, Vec<f64>) {
    let mut temperature = init_temp;
    let mut couts: Vec<f64> = Vec::new();
    let mut iter: i32 = 0;
    let mut old = plan;
    let mut new = plan;
    while iter <= max_iter && temperature > 1f64 {
        couts.push(cost_of_travel_plan(&old, &cities));
        iter = iter + 1;
        new = shuffle(&old, &iter);
        if accept(&new, &old, &temperature, &cities) {
            old = new;
        }
        temperature = init_temp * (-(iter as f64)/(max_iter as f64)*cooling_speed).exp();
    }
    (new, couts)
}

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