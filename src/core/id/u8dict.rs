pub struct U8Dict;

impl U8Dict {
    /// Encode a word back into its dictionary index (0–255)
    pub fn encode(word: &str) -> u8 {
        crate::core::id::user_words::WORDS
            .iter()
            .position(|w| *w == word)
            .unwrap_or(0) as u8
    }
}
