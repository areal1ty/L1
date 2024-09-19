use std::collections::HashSet;

pub fn run(string: &str) -> bool {
    let mut seen = HashSet::new();
    for c in string.chars().map(|c| c.to_lowercase().next().unwrap()) {
        if !seen.insert(c) {
            return false;
        }
    }
    true
}