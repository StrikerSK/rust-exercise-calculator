use crate::calculations::exercise_calculator::ExerciseCalculator;
use crate::inputs;

pub fn calculate_exercise_1rm(calculator: &dyn ExerciseCalculator) {
    println!("Please enter used reps count");
    let reps = inputs::get_input_integer();
    println!("Please enter used weight");
    let weight = inputs::get_input_float();
    println!("Your one rep max is: {}", calculator.calculate_1rm(reps, weight));
}

pub fn calculate_exercise_nrm(calculator: &dyn ExerciseCalculator) {
    println!("Please enter your PR");
    let pr = inputs::get_input_float();
    println!("Please enter expected weight");
    let weight = inputs::get_input_float();
    println!("For weight {} kgs, you should execute: {} reps", weight, calculator.calculate_nrm(pr, weight));
}

pub fn calculate_exercise_weight(calculator: &dyn ExerciseCalculator) {
    println!("Please enter your PR");
    let pr = inputs::get_input_float();
    println!("Please enter expected reps count");
    let reps = inputs::get_input_integer();
    println!("Your training weight is: {}", calculator.calculate_used_weight(pr, reps));
}