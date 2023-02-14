use serde::{Serialize, Deserialize};

use crate::{
    armor::armor_item::builder::base::{MundaneArmorBuilder, MundaneArmorBuilderWithWeightClass},
    CharacterMutation,
};

use super::{MundaneArmor, MundaneArmorName};

/// The name and properties of a piece of mundane armor to be added to a
/// character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddMundaneArmor {
    pub(crate) name: MundaneArmorName,
    pub(crate) armor: MundaneArmor,
}

impl AddMundaneArmor {
    /// Starts constructing a new piece of armor.
    pub fn name(name: impl Into<MundaneArmorName>) -> MundaneArmorBuilder {
        MundaneArmorBuilder::name(name)
    }
}

impl From<MundaneArmorBuilderWithWeightClass> for AddMundaneArmor {
    fn from(builder: MundaneArmorBuilderWithWeightClass) -> Self {
        builder.build()
    }
}

impl From<AddMundaneArmor> for CharacterMutation {
    fn from(add_mundane_armor: AddMundaneArmor) -> Self {
        Self::AddMundaneArmor(add_mundane_armor)
    }
}
