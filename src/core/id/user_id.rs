use serde::{Serialize, Deserialize};
use std::fmt;
use crate::core::id::u8dict::U8Dict;

/// A human-friendly 48-bit user ID:
/// five word indices + a number (0-255)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId {
    pub words: [u8; 5],
    pub num: u8,
}

impl UserId {
    /// Construct from components
    pub fn new(words: [u8; 5], num: u8) -> Self {
        Self { words, num }
    }

    /// Generate a random ID
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            words: [
                rng.random::<u8>(),
                rng.random::<u8>(),
                rng.random::<u8>(),
                rng.random::<u8>(),
                rng.random::<u8>(),
            ],
            num: rng.random::<u8>(),
        }
    }

}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // IMPORTANT: WORDS must be provided somewhere else.
        // We'll set this up in Step 2.
        write!(
            f,
            "{}-{}-{}-{}-{}-{:03}",
            crate::core::id::user_words::WORDS[self.words[0] as usize],
            crate::core::id::user_words::WORDS[self.words[1] as usize],
            crate::core::id::user_words::WORDS[self.words[2] as usize],
            crate::core::id::user_words::WORDS[self.words[3] as usize],
            crate::core::id::user_words::WORDS[self.words[4] as usize],
            self.num,
        )
    }
}

impl UserId {
    pub fn zero() -> Self {
        Self { words: [0; 5], num: 0 }
    }
}

impl UserId {
    pub fn from_string(s: &str) -> Self {
        // reverse of .to_string(): 5 words + number
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 6 { return UserId::zero(); }

        let words = [
            U8Dict::encode(parts[0]),
            U8Dict::encode(parts[1]),
            U8Dict::encode(parts[2]),
            U8Dict::encode(parts[3]),
            U8Dict::encode(parts[4]),
        ];

        let num = parts[5].parse().unwrap_or(0);

        UserId::new(words, num)
    }
}

impl UserId {
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        let bytes = uuid.as_bytes();

        UserId {
            words: [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]],
            num: bytes[5],
        }
    }
}
