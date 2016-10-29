// load travel module
mod travel;

// for convenient calls
use travel::*;

fn main() {
    println!("Salesmen are travelling ...");
    let destination = format!("Chicago");
    hello_from_travel(&destination);
}
