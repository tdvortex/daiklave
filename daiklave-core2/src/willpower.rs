use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Willpower {
    pub(crate) current: u8,
    pub(crate) rating: u8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self {
            current: 3,
            rating: 3,
        }
    }
}

impl Willpower {
    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn rating(&self) -> u8 {
        self.rating
    }
}
