#![feature(test)]
extern crate test;
extern crate csv;
extern crate rand;
extern crate rustc_serialize;
extern crate rayon;

// load travel module
mod travel;
// for convenient calls
use travel::*;
use rayon::prelude::*;

fn main() {
    // Welcome
    println!("Salesmen are travelling ...");

    // load resources, IO serve as interface
    let cities: Vec<City> = load_cities_from_file("./ressources/cities.csv");
    let plan = load_travel_plan_from_file("./ressources/travel-plan.txt");

    // more Welcome
    hello_from_travel(name_to_city(&plan[0], &cities));

    // compute starting values
    let cost = cost_of_travel_plan(&plan, &cities);
    println!("The initial travel plan is: {:?}.", plan);
    println!("Its cost is: {} km.", cost);
    // // optimize and print resulst
    // println!{"\nOptimizing the travel plan...\n"}
    // let optimized = optimize_travel(&plan, &cities, 2000f64, 4f64, 1000);
    // print!("The optimized travel plan is: {:?}.", optimized.0);
    // println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);
    //
    // // another run optimize and print resulst
    // println!{"\nOptimizing the travel plan...\n"}
    // let optimized = optimize_travel(&optimized.0, &cities, 2000f64, 4f64, 1000);
    // print!("The optimized travel plan is: {:?}.", optimized.0);
    // println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);
    //
    // // another run optimize and print resulst
    // println!{"\nOptimizing the travel plan...\n"}
    // let optimized = optimize_travel(&optimized.0, &cities, 2000f64, 4f64, 1000);
    // print!("The optimized travel plan is: {:?}.", optimized.0);
    // println!("Its cost is: {} km.", optimized.1[optimized.1.len() - 1]);

    // experiment with rayon for parallel execution for different temperatures
    let temps = [10f64, 50f64, 100f64, 500f64, 800f64, 1000f64, 2000f64, 3000f64, 4000f64,
                 5000f64, 6000f64, 10000f64];
    let mut results = vec![];
    temps.par_iter()
        .map(|&i| optimize_travel(&plan, &cities, i, 4f64, 1000))
        .collect_into(&mut results);
    for i in 0..temps.len() {
        println!("starting temperature: {}", temps[i]);
        println!("Optimized plan: {:?}, cost: {:2}",
                 results[i].0,
                 results[i].1[results[i].1.len() - 1]);
    }
}
