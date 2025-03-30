use regex::Regex;

pub fn validate_username(username: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    re.is_match(username)
}

pub fn validate_password(password: &str) -> bool {
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_special = password.chars().any(|c| "@#_-&?!".contains(c));
    let has_min_length = password.len() >= 8;

    has_lowercase && has_uppercase && has_special && has_min_length
}