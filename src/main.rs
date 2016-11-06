extern crate csv;
extern crate rand;
extern crate rustc_serialize;

// load travel module
mod travel;

// for convenient calls
use travel::*;

fn main() {
    println!("Salesmen are travelling ...");

    let cities: Vec<City> = load_cities_from_file("./ressources/cities.csv");
    let plan = load_travel_plan_from_file("./ressources/travel-plan.txt");
    let cost = cost_of_travel_plan(&plan, &cities);
    println!("Total distance of initial travel plan: {} km", cost);

    println!{"Optimizing the travel plan..."}
    let optimized = optimize_travel(&plan, &cities, 2000f64, 4f64, 1000);
    println!("The optimized travel plan is: {:?}.", optimized.0);
    println!("Its cost is: {} km.", optimized.1[optimized.1.len()-1]);
}
