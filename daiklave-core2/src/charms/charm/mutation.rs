use crate::{
    exaltation::exalt::exalt_type::solar::charm::AddSolarCharm,
    martial_arts::charm::AddMartialArtsCharm, sorcery::spell::AddSpell,
};

use super::{evocation::AddEvocation, spirit::AddEclipseCharm};

/// A Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddCharm {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(AddEclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(AddEvocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(AddMartialArtsCharm),
    /// A Solar charm.
    Solar(AddSolarCharm),
    /// A Spell.
    Spell(AddSpell),
}
