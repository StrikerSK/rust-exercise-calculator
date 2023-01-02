use crate::calculator::calculate_used_weight;

mod calculator;

fn main() {
    println!("Hello user! I can calculate for you 1-RM, N-RM and Training weight!");
    println!("Which one would you like to calculate?");

    println!("How many reps done?");
    let mut reps: i32 = get_input_integer();
    println!("Scanned reps value: {}", reps);

    println!("How many weight used?");
    let mut weight: f32 = get_input_float();
    println!("Scanned weight value: {}", weight);

    println!("What is your 1RM?");
    let mut pr: f32 = get_input_float();
    println!("Scanned pr value: {}", pr);

    println!("Expected 1 rep max weight: {}", calculator::calculate_1rm(reps, weight));
    println!("Expected reps for {} kgs value: {}", weight, calculator::calculate_nrm(pr, weight));
    println!("Expected {} rep weight: {}", reps, calculator::calculate_used_weight(pr, reps));
}

fn get_input_string() -> String {
    let mut input_text: String = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    return input_text.trim().to_string();
}

fn get_input_integer() -> i32 {
    match get_input_string().parse::<i32>() {
        Ok(i) => i,
        Err(..) => panic!("This element was not of Integer type"),
    }
}

fn get_input_float() -> f32 {
    match get_input_string().parse::<f32>() {
        Ok(i) => i,
        Err(..) => panic!("This element was not of Float type"),
    }
}
