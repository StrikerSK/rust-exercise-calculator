mod calculator;

fn main() {
    println!("Hello user! I can calculate for you 1-RM, N-RM and Weight!");
    println!("Which one would you like to calculate (Max, Reps, Weight)?");

    loop {
        let binding = get_input_string().to_lowercase();
        let calc_type = binding.as_str();
        match calc_type {
            "max" => {
                println!("Please enter used reps count");
                let reps = get_input_integer();
                println!("Please enter used weight");
                let weight = get_input_float();
                println!("Your one rep max is: {}", calculator::calculate_1rm(reps, weight));
                break;
            },

            "reps" => {
                println!("Please enter your PR");
                let pr = get_input_float();
                println!("Please enter expected weight");
                let weight = get_input_float();
                println!("For weight {} kgs, you should execute: {} reps", weight, calculator::calculate_nrm(pr, weight));
                break;
            },

            "weight" => {
                println!("Please enter your PR");
                let pr = get_input_float();
                println!("Please enter expected reps count");
                let reps = get_input_integer();
                println!("Your training weight is: {}", calculator::calculate_used_weight(pr, reps));
                break;
            },

            _ => println!("Sorry, I don't know this calculation type! Do you want to calculate: Max, Reps or Weight"),
        }
    }
}

fn get_input_string() -> String {
    let mut input_text: String = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    return input_text.trim().to_string();
}

fn get_input_integer() -> i32 {
    let mut input: String = get_input_string();
    loop {
        match input.parse::<i32>() {
            Ok(i) => { return i },
            Err(..) => {
                println!("This element was not of Integer type! Please try again!");
                input = get_input_string();
            },
        }
    }
}

fn get_input_float() -> f32 {
    let mut input: String = get_input_string();
    loop {
        match input.parse::<f32>() {
            Ok(i) => { return i },
            Err(..) => {
                println!("This element was not of Float type! Please try again!");
                input = get_input_string();
            },
        }
    }
}
