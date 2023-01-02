mod calculator;

fn main() {
    println!("Hello user! I can calculate for you 1-RM, N-RM and Weight!");
    println!("Which one would you like to calculate (Max, Reps, Weight)?");

    let binding = get_input_string().to_lowercase();
    let calc_type = binding.as_str();
    match calc_type {

        "max" => {
            println!("Please enter used reps count");
            let reps = get_input_integer();
            println!("Please enter used weight");
            let weight = get_input_float();
            println!("Your one rep max is: {}", calculator::calculate_1rm(reps, weight));
        },

        "reps" => {
            println!("Please enter your PR");
            let pr = get_input_float();
            println!("Please enter expected weight");
            let weight = get_input_float();
            println!("For weight {} kgs, you should execute: {} reps", weight, calculator::calculate_nrm(pr, weight));
        },

        "weight" => {
            println!("Please enter your PR");
            let pr = get_input_float();
            println!("Please enter expected reps count");
            let reps = get_input_integer();
            println!("Your training weight is: {}", calculator::calculate_used_weight(pr, reps));
        },

        _ => println!("Sorry, I don't know this calculation type!"),
    }
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
