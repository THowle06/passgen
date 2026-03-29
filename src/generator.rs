use rand::RngExt;

pub struct PasswordPolicy {
    pub length: usize,
    pub include_upper: bool,
    pub include_lower: bool,
    pub include_digits: bool,
    pub include_symbols: bool,
    pub require_each_class: bool,
}

pub fn generate_password(policy: &PasswordPolicy) -> String {
    let mut charset = Vec::new();

    if policy.include_upper {
        charset.extend('A'..='Z');
    }
    if policy.include_lower {
        charset.extend('a'..='z');
    }
    if policy.include_digits {
        charset.extend('0'..='9');
    }
    if policy.include_symbols {
        charset.extend([
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=',
        ]);
    }

    let mut rng = rand::rng();
    let mut password = Vec::new();

    // TODO: enforce require_each_class
    for _ in 0..policy.length {
        let idx = rng.random_range(0..charset.len());
        password.push(charset[idx]);
    }

    password.iter().collect()
}
