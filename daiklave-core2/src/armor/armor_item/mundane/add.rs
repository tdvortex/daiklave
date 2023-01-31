use std::{collections::HashSet};

use crate::{armor::armor_item::builder::base::BaseArmorItemBuilder, CharacterMutation};

use super::{MundaneArmor, MundaneArmorName};

/// The name and properties of a piece of mundane armor to be added to a
/// character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMundaneArmor {
    name: MundaneArmorName,
    armor: MundaneArmor,
}

impl AddMundaneArmor {
    pub fn builder(name: impl Into<String>) -> BaseArmorItemBuilder {
        BaseArmorItemBuilder {
            name: name.into(),
            book_reference: None,
            tags: HashSet::new(),
        }
    }
}

impl From<AddMundaneArmor> for CharacterMutation {
    fn from(add_mundane_armor: AddMundaneArmor) -> Self {
        Self::AddMundaneArmor(add_mundane_armor)
    }
}