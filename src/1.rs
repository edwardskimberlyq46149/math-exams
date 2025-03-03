use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Add some scores to the map
    scores.insert(95, "Excellent");
    scores.insert(80, "Good");
    scores.insert(70, "Fair");
    scores.insert(60, "Poor");

    // Print out the scores in a random order
    for (score, grade) in &scores {
        println!("{} {}", score, grade);
    }
}
