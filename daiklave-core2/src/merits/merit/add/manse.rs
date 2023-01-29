use crate::{merits::merit::{DemenseName, ManseName}, hearthstones::hearthstone::AddHearthstone, CharacterMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddManse {
    pub manse_name: ManseName,
    pub demense_name: DemenseName,
    pub add_hearthstone: AddHearthstone,
}

impl From<AddManse> for CharacterMutation {
    fn from(add_manse: AddManse) -> Self {
        Self::AddMerit(add_manse.into())
    }
}