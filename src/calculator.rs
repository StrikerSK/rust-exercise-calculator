pub fn calculate_nrm(pr: f32, weight: f32) -> f32 {
    return (pr - weight) / (weight * 0.0333)
}

pub fn calculate_used_weight(pr: f32, reps: i32) -> f32 {
    return pr / ((reps as f32 * 0.0333) + 1.0);
}

 pub fn calculate_1rm(reps: i32, weight: f32) -> f32 {
     return reps as f32 * weight *  0.0333 + weight
 }