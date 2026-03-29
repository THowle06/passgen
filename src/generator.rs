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

    if all_chars.is_empty() {
        panic!("No character sets enabled");
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

pub fn calculate_entropy(policy: &PasswordPolicy) -> f64 {
    let mut charset_size = 0;

    if policy.include_upper {
        charset_size += 26;
    }
    if policy.include_lower {
        charset_size += 26;
    }
    if policy.include_digits {
        charset_size += 10;
    }
    if policy.include_symbols {
        charset_size += 14;
    }

    (policy.length as f64) * (charset_size as f64).log2()
}

pub fn classify_entropy(entropy: f64) -> &'static str {
    match entropy {
        e if e < 40.0 => "Weak",
        e if e < 80.0 => "Moderate",
        _ => "Strong",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_policy() -> PasswordPolicy {
        PasswordPolicy {
            length: 16,
            include_upper: true,
            include_lower: true,
            include_digits: true,
            include_symbols: true,
            require_each_class: false,
        }
    }

    #[test]
    fn no_symbols_when_disabled() {
        let mut policy = default_policy();
        policy.include_symbols = false;

        let pwd = generate_password(&policy);

        assert!(pwd.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn includes_all_classes_when_required() {
        let mut policy = default_policy();
        policy.require_each_class = true;

        let pwd = generate_password(&policy);

        assert!(pwd.chars().any(|c| c.is_ascii_uppercase()));
        assert!(pwd.chars().any(|c| c.is_ascii_lowercase()));
        assert!(pwd.chars().any(|c| c.is_ascii_digit()));
        assert!(pwd.chars().any(|c| "!@#$%^&*()-_+=?".contains(c)));
    }

    #[test]
    fn works_with_minimum_length_require_each_class() {
        let policy = PasswordPolicy {
            length: 4,
            include_upper: true,
            include_lower: true,
            include_digits: true,
            include_symbols: true,
            require_each_class: true,
        };

        let pwd = generate_password(&policy);
        assert_eq!(pwd.len(), 4);
    }
}
