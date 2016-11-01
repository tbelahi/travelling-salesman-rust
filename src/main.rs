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
    
    let plan = load_travel_plan_from_file("./ressources/travel-plan.txt");
    println!("travel plan: {:?}", plan);
    let nantes = name_to_city(format!("Nantes"), &cities);
    println!("{:?}", nantes);
    let villes: Vec<&City> = plan.into_iter().map(|x| name_to_city(x, &cities)).collect();
    println!("{:?}", villes);
    //let cout = zip(villes[0..-1], villes[1..]).fold(0, |&mut acc(x, y| acc +  x.distance(y)).unwrap();
    let len = villes.len();
    let mut cout: f64 = 0f64;
    for i in 0..len-1 {
        println!("la distance entre {} et {} est {} km", 
                villes[i].name, villes[i+1].name, villes[i].distance(&villes[i+1]));
        cout = cout + villes[i].distance(villes[i+1])
    }
}
