pub fn calculate_1rm(reps: i32, weight: f32) -> f32 {
    return reps as f32 * weight *  0.0333 + weight
}

pub fn calculate_nrm(pr: f32, weight: f32) -> f32 {
    return (pr - weight) / (weight * 0.0333)
}

pub fn calculate_used_weight(pr: f32, reps: i32) -> f32 {
    return pr / ((reps as f32 * 0.0333) + 1.0);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pr_calculation() {
        assert_eq!(calculate_1rm(5, 100.0), 116.65);
    }

    #[test]
    fn test_n_rm() {
        assert_eq!(calculate_nrm(116.65, 100.0), 5.0);
    }

    #[test]
    fn test_used_weight() {
        assert_eq!(calculate_used_weight(116.65, 5), 100.0);
    }
}
