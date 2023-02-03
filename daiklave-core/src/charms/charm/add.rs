use crate::{
    exaltation::exalt::exalt_type::solar::charm::AddSolarCharm,
    martial_arts::charm::AddMartialArtsCharm, sorcery::spell::AddSpell, CharacterMutation,
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

impl From<AddEclipseCharm> for AddCharm {
    fn from(add_eclipse: AddEclipseCharm) -> Self {
        Self::Eclipse(add_eclipse)
    }
}

impl From<AddEvocation> for AddCharm {
    fn from(add_evocation: AddEvocation) -> Self {
        Self::Evocation(add_evocation)
    }
}

impl From<AddMartialArtsCharm> for AddCharm {
    fn from(add_ma_charm: AddMartialArtsCharm) -> Self {
        Self::MartialArts(add_ma_charm)
    }
}

impl From<AddSolarCharm> for AddCharm {
    fn from(add_solar_charm: AddSolarCharm) -> Self {
        Self::Solar(add_solar_charm)
    }
}

impl From<AddSpell> for AddCharm {
    fn from(add_spell: AddSpell) -> Self {
        Self::Spell(add_spell)
    }
}

impl From<AddCharm> for CharacterMutation {
    fn from(add_charm: AddCharm) -> Self {
        Self::AddCharm(add_charm)
    }
}
