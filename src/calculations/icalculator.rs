trait ExerciseCalculator {
    fn calculate_1rm(&self, reps: i32, weight: f32) -> f32;
    fn calculate_nrm(&self, pr: f32, weight: f32) -> i32;
    fn calculate_used_weight(&self, pr: f32, reps: i32) -> f32;
}