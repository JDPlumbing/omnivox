use rand::Rng;

/// Generate a random droid-style ID (e.g. "R2-D2", "X9C3").
pub fn generate() -> String {
    let mut rng = rand::rng(); // replaces thread_rng()
    let len = rng.random_range(4..=6); // replaces gen_range
    let mut chars: Vec<char> = Vec::new();

    let charset: Vec<char> = ('A'..='Z')
        .chain('a'..='z')
        .chain('0'..='9')
        .collect();

    for _ in 0..len {
        let c = charset[rng.random_range(0..charset.len())];
        chars.push(c);
    }

    // Maybe insert a dash (not at first or last position)
    if len > 3 && rng.random_bool(0.5) {
        let pos = rng.random_range(1..len - 1);
        chars.insert(pos, '-');
    }

    chars.iter().collect()
}
