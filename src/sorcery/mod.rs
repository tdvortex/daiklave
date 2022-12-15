use serde::{Serialize, Deserialize};

use crate::charms::Spell;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum SorcererCircle {
    Terrestrial,
    Celestial,
    Solar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorcererTraits {
    circle: SorcererCircle,
    initiation: String,
    shaping_rituals: Vec<String>,
    control_spell: Spell,
    other_spells: Vec<Spell>,
}