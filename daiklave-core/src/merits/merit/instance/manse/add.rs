use crate::{
    hearthstones::hearthstone::AddHearthstone,
    merits::merit::{instance::DemenseName, AddMerit},
    CharacterMutation,
};

use super::{builder::ManseBuilder, name::ManseName};

/// A mutation to add a Manse, and its associated demense and hearthstone, to
/// a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddManse {
    pub(crate) manse_name: ManseName,
    pub(crate) demense_name: DemenseName,
    pub(crate) hearthstone: AddHearthstone,
}

impl AddManse {
    /// Starts constructing a manse with the given name.
    pub fn name(name: impl Into<ManseName>) -> ManseBuilder {
        ManseBuilder::name(name)
    }
}

impl From<AddManse> for CharacterMutation {
    fn from(add_manse: AddManse) -> Self {
        AddMerit::from(add_manse).into()
    }
}
