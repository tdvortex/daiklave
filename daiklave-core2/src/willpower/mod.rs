mod error;
mod gain;
mod set_rating;
mod spend;
use std::num::NonZeroU8;

pub use error::WillpowerError;
pub use gain::GainWillpower;
pub use spend::SpendWillpower;
pub use set_rating::SetWillpowerRating;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Willpower {
    pub(crate) current: u8,
    pub(crate) rating: NonZeroU8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self {
            current: 3,
            rating: NonZeroU8::new(3).unwrap(),
        }
    }
}

impl Willpower {
    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn rating(&self) -> NonZeroU8 {
        self.rating
    }
}
