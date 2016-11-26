#![feature(test)]
extern crate test;

extern crate csv;
extern crate rand;
extern crate rustc_serialize;


// load travel module
mod travel;

// for convenient calls
use travel::*;

fn main() {
    // Welcome
    println!("Salesmen are travelling ...");

    // load resources, IO serve as interface
    let cities: Vec<City> = load_cities_from_file("./ressources/cities.csv");
    let plan = load_travel_plan_from_file("./ressources/travel-plan.txt");

    // compute starting values
    let cost = cost_of_travel_plan(&plan, &cities);
    println!("The initial travel plan is: {:?}.", plan);
    println!("Its cost is: {} km.", cost);

    // optimize and print resulst
    println!{"\nOptimizing the travel plan...\n"}
    let optimized = optimize_travel(&plan, &cities, 2000f64, 4f64, 1000);
    print!("The optimized travel plan is: {:?}.", optimized.0);
    println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);

    // another run optimize and print resulst
    println!{"\nOptimizing the travel plan...\n"}
    let optimized = optimize_travel(&optimized.0, &cities, 2000f64, 4f64, 1000);
    print!("The optimized travel plan is: {:?}.", optimized.0);
    println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);

    // another run optimize and print resulst
    println!{"\nOptimizing the travel plan...\n"}
    let optimized = optimize_travel(&optimized.0, &cities, 2000f64, 4f64, 1000);
    print!("The optimized travel plan is: {:?}.", optimized.0);
    println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);
}
