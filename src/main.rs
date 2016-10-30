extern crate csv;
extern crate rustc_serialize;

// load travel module
mod travel;

// for convenient calls
use travel::*;

fn main() {
    println!("Salesmen are travelling ...");
    let destination = City {name: format!("Chicago"), lon:-87.63, lat: 41.87 };
    hello_from_travel(&destination);

    let paris = City { name: "Paris".to_string(), lon: 2.3476, lat: 48.8543 };
    let marseille = City { name: "Marseille".to_string(), lon: 5.3700, lat: 43.2948 };

    println!("{:?}", paris);
    println!("{:?}", marseille);
    let d = paris.distance(&marseille);
    println!("Distance de {} à {}: {:?}", paris.name, marseille.name, d);

    let cities: Vec<City> = load_cities_from_file("./ressources/cities.csv");
    println!("{} est située aux coordonnées :", cities[5].name);
    println!("longitude: {}", cities[5].lon);
    println!("latitude: {}", cities[5].lat);
    println!("la distance entre {} et {} est {} km", 
                cities[6].name, cities[8].name, cities[6].distance(&cities[8]));

}
