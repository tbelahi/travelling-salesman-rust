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
}
