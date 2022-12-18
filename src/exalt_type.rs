use serde::{Deserialize, Serialize};

use crate::{solar::SolarTraits, sorcery::MortalSorcererLevel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExaltType {
    Mortal(MortalSorcererLevel),
    Solar(SolarTraits),
}

impl Default for ExaltType {
    fn default() -> Self {
        Self::Mortal(MortalSorcererLevel::default())
    }
}
