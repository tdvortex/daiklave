use std::collections::HashSet;

use crate::armor::armor_item::builder::base::BaseArmorItemBuilder;

use super::{MundaneArmor, MundaneArmorName};

/// The name and properties of a piece of mundane armor to be added to a
/// character.
pub struct AddMundaneArmor {
    name: MundaneArmorName,
    armor: MundaneArmor,
}

impl AddMundaneArmor {
    pub fn builder(name: impl ToString) -> BaseArmorItemBuilder {
        BaseArmorItemBuilder {
            name: name.to_string(),
            book_reference: None,
            tags: HashSet::new(),
        }
    }
}