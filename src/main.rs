use crate::calculator::calculate_used_weight;

mod calculator;

fn main() {
    println!("Hello, world!");

    let pr = calculator::calculate_1rm(5, 100.0);

    println!("1 rep max value: {}", pr);
    println!("N rep max value: {}", calculator::calculate_nrm(pr, 100.0));
    println!("Recommended weight value: {}", calculate_used_weight(pr, 5));
}
