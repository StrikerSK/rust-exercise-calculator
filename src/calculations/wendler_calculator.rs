use crate::calculations::exercise_calculator::ExerciseCalculator;

pub(crate) struct WendlerCalculator;

impl ExerciseCalculator for WendlerCalculator {
    fn calculate_1rm(&self, reps: i32, weight: f32) -> f32 {
        return reps as f32 * weight *  0.0333 + weight
    }

    fn calculate_nrm(&self, pr: f32, weight: f32) -> f32 {
        return (pr - weight) / (weight * 0.0333)
    }

    fn calculate_used_weight(&self, pr: f32, reps: i32) -> f32 {
        return pr / ((reps as f32 * 0.0333) + 1.0);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use super::WendlerCalculator;

    #[test]
    fn test_pr_calculation() {
        assert_eq!(WendlerCalculator.calculate_1rm(5, 100.0), 116.65);
    }

    #[test]
    fn test_n_rm() {
        assert_eq!(WendlerCalculator.calculate_nrm(116.65, 100.0), 5.0);
    }

    #[test]
    fn test_used_weight() {
        assert_eq!(WendlerCalculator.calculate_used_weight(116.65, 5), 100.0);
    }
}
