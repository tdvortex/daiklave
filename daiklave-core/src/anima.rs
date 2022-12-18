use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AnimaLevel {
    Dim,
    Glowing,
    Burning,
    Bonfire,
}

impl AnimaLevel {
    pub fn increase(&mut self) {
        let new_rating = match self {
            AnimaLevel::Dim => AnimaLevel::Glowing,
            AnimaLevel::Glowing => AnimaLevel::Burning,
            AnimaLevel::Burning => AnimaLevel::Bonfire,
            AnimaLevel::Bonfire => AnimaLevel::Bonfire,
        };
        *self = new_rating;
    }

    pub fn decrease(&mut self) {
        let new_rating = match self {
            AnimaLevel::Dim => AnimaLevel::Dim,
            AnimaLevel::Glowing => AnimaLevel::Dim,
            AnimaLevel::Burning => AnimaLevel::Glowing,
            AnimaLevel::Bonfire => AnimaLevel::Burning,
        };
        *self = new_rating;
    }
}
