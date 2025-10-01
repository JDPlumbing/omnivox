use rand::Rng;
use std::collections::HashSet;
use uuid::Uuid;
use sha2::{Digest, Sha256};

/// Generate a short code (6 chars) from a UUID by hashing & truncating
pub fn short_code_from_uuid(uuid: &Uuid) -> String {
    let mut hasher = Sha256::new();
    hasher.update(uuid.as_bytes());
    let hash = hasher.finalize();

    // Take first 30 bits (~6 base62 chars worth)
    let mut num: u64 = ((hash[0] as u64) << 24)
        | ((hash[1] as u64) << 16)
        | ((hash[2] as u64) << 8)
        | (hash[3] as u64);

    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();

    let mut code = String::new();
    for _ in 0..6 {
        code.push(chars[(num % 62) as usize]);
        num /= 62;
    }
    code
}

/// Ensure uniqueness in a namespace by retrying or tweaking with dash/underscore
pub fn unique_short_code(uuid: &Uuid, existing: &mut HashSet<String>) -> String {
    let code = short_code_from_uuid(uuid);

    if !existing.contains(&code) {
        existing.insert(code.clone());
        return code;
    }

    // Collision: insert dash/underscore in a random middle position
    let mut rng = rand::rng();
    let insert_pos = rng.random_range(1..code.len()); // never first or last
    let tweak = if rng.random_bool(0.5) { '-' } else { '_' };

    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(insert_pos, tweak);
    let tweaked: String = chars.into_iter().collect();

    existing.insert(tweaked.clone());
    tweaked
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[test]
    fn short_code_has_six_chars() {
        let uuid = Uuid::new_v4();
        let code = short_code_from_uuid(&uuid);
        assert_eq!(code.len(), 6);
    }

    #[test]
    fn unique_short_code_is_unique() {
        let uuid = Uuid::new_v4();
        let mut existing = HashSet::new();

        let code1 = unique_short_code(&uuid, &mut existing);
        let code2 = unique_short_code(&uuid, &mut existing);

        assert_ne!(code1, code2, "two generated codes should differ if collision");
    }
}
