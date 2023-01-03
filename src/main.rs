use calculations::{calculator_runner, exercise_calculator};

mod inputs;
mod calculations;

fn main() {
    println!("Hello user! I can calculate for you 1-RM, N-RM and Weight of your exercise!");
    println!("Which one would you like to calculate (Max, Reps, Weight)?");

    loop {
        let binding = inputs::get_input_string().to_lowercase();
        let calc_type = binding.as_str();

        // '&' solves 'Sized' issue with instantiating int trait object
        let calculator: &dyn exercise_calculator::ExerciseCalculator = &calculations::wendler_calculator::WendlerCalculator {};

        match calc_type {
            "max" => {
                calculator_runner::calculate_exercise_1rm(calculator);
                break;
            },

            "reps" => {
                calculator_runner::calculate_exercise_nrm(calculator);
                break;
            },

            "weight" => {
                calculator_runner::calculate_exercise_weight(calculator);
                break;
            },

            _ => println!("Sorry, I don't know this calculation type! Do you want to calculate: Max, Reps or Weight"),
        }
    }
}
