use crate::{sorcery::archetype::SorceryArchetypeName};

use super::ShapingRitualDetails;

/// A shaping ritual to add to a character.
pub struct AddShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) summary: String,
    pub(crate) ritual: ShapingRitualDetails
}