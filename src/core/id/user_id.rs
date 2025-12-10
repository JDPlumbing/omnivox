use serde::{Serialize, Deserialize};
use std::fmt;

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
