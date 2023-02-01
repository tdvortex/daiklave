mod mutation;
pub(crate) use mutation::ArmorNameMutation;

use super::{EquipArmor, remove::RemoveArmor};

/// The name of a piece of armor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmorName<'source> {
    /// Mundane, non-artifact armor.
    Mundane(&'source str),
    /// Artifact armor. This is the name for the specific piece of armor (like
    /// "Brilliant Sentinel"), not the generic item name (like "Articulated
    /// Plate (Artifact)").
    Artifact(&'source str),
}

impl<'source> ArmorName<'source> {
    /// Constructs a mutation to equip this armor.
    pub fn equip(self) -> EquipArmor {
        EquipArmor(self.into())
    }

    /// Constructs a mutation to remove this armor from a character.
    pub fn remove(self) -> RemoveArmor {
        match self {
            ArmorName::Mundane(name) => RemoveArmor::Mundane(name.into()),
            ArmorName::Artifact(name) => RemoveArmor::Artifact(name.into()),
        }
    }
}