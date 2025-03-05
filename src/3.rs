use std::collections::HashMap;

fn main() {
    let mut exams = HashMap::new();

    exams.insert("Math 1", "2023");
    exams.insert("Math 2", "2024");
    exams.insert("Math 3", "2025");
    exams.insert("Math 4", "2026");
    exams.insert("Math 5", "2027");
    exams.insert("Math 6", "2028");

    for (key, value) in &exams {
        println!("{}: {}", key, value);
    }
}
