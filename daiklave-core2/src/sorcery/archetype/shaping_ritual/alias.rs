use crate::sorcery::archetype::SorceryArchetypeName;

use super::ShapingRitual;

/// A short description of a shaping ritual
pub type ShapingRitualSummary = String;

/// A shaping ritual to add to a character.
pub type AddShapingRitual = (SorceryArchetypeName, ShapingRitualSummary, ShapingRitual);
