use calculations::exercise_calculator;

mod inputs;
mod calculations;

fn main() {
    println!("Hello user! I can calculate for you 1-RM, N-RM and Weight of your exercise!");
    println!("Which one would you like to calculate (Max, Reps, Weight)?");

    loop {
        // '&' solves issue with instantiating int trait object
        let calculator: &dyn exercise_calculator::ExerciseCalculator = &calculations::wendler_calculator::WendlerCalculator {};
        let binding = inputs::get_input_string().to_lowercase();
        let calc_type = binding.as_str();
        match calc_type {
            "max" => {
                println!("Please enter used reps count");
                let reps = inputs::get_input_integer();
                println!("Please enter used weight");
                let weight = inputs::get_input_float();
                println!("Your one rep max is: {}", calculator.calculate_1rm(reps, weight));
                break;
            },

            "reps" => {
                println!("Please enter your PR");
                let pr = inputs::get_input_float();
                println!("Please enter expected weight");
                let weight = inputs::get_input_float();
                println!("For weight {} kgs, you should execute: {} reps", weight, calculator.calculate_nrm(pr, weight));
                break;
            },

            "weight" => {
                println!("Please enter your PR");
                let pr = inputs::get_input_float();
                println!("Please enter expected reps count");
                let reps = inputs::get_input_integer();
                println!("Your training weight is: {}", calculator.calculate_used_weight(pr, reps));
                break;
            },

            _ => println!("Sorry, I don't know this calculation type! Do you want to calculate: Max, Reps or Weight"),
        }
    }
}
