use rand::{RngExt, seq::SliceRandom};

pub struct PasswordPolicy {
    pub length: usize,
    pub include_upper: bool,
    pub include_lower: bool,
    pub include_digits: bool,
    pub include_symbols: bool,
    pub require_each_class: bool,
}

pub fn generate_password(policy: &PasswordPolicy) -> String {
    let mut rng = rand::rng();

    let upper: Vec<char> = ('A'..='Z').collect();
    let lower: Vec<char> = ('a'..='z').collect();
    let digits: Vec<char> = ('0'..='9').collect();
    let symbols: Vec<char> = vec![
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '?',
    ];

    let mut all_chars = Vec::new();
    let mut password = Vec::new();

    // Build enabled sets
    let mut enabled_sets: Vec<&Vec<char>> = Vec::new();

    if policy.include_upper {
        all_chars.extend(&upper);
        enabled_sets.push(&upper);
    }
    if policy.include_lower {
        all_chars.extend(&lower);
        enabled_sets.push(&lower);
    }
    if policy.include_digits {
        all_chars.extend(&digits);
        enabled_sets.push(&digits);
    }
    if policy.include_symbols {
        all_chars.extend(&symbols);
        enabled_sets.push(&symbols);
    }

    // Ensure at least one from each class
    if policy.require_each_class {
        for set in &enabled_sets {
            let ch = set[rng.random_range(0..set.len())];
            password.push(ch);
        }
    }

    // Fill remaining characters
    while password.len() < policy.length {
        let ch = all_chars[rng.random_range(0..all_chars.len())];
        password.push(ch);
    }

    // Shuffle to avoid predictable placement
    password.shuffle(&mut rng);

    password.iter().collect()
}
