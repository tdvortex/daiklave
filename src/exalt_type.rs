use serde::{Deserialize, Serialize};

use crate::{solar::SolarTraits, sorcery::{MortalSorcererLevel, Sorcerer}};

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

impl Sorcerer for ExaltType {
    fn shaping_rituals(&self) -> Option<Vec<&crate::sorcery::ShapingRitual>> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_level) => mortal_sorcerer_level.shaping_rituals(),
            ExaltType::Solar(solar_traits) => solar_traits.shaping_rituals(),
        }
    }

    fn spells(&self) -> Option<Vec<(&crate::charms::Spell, bool)>> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_level) => mortal_sorcerer_level.spells(),
            ExaltType::Solar(solar_traits) => solar_traits.spells(),
        }
    }
}