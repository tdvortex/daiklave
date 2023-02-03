use crate::{
    hearthstones::HearthstoneError,
    merits::merit::{
        manse::{
            builder::{ManseBuilder, ManseBuilderWithHearthstone},
            ManseName,
        },
        AddMerit,
    },
    CharacterMutation,
};

use super::{builder::HearthstoneBuilder, HearthstoneName, HearthstoneTemplate};

/// A hearthstone and its name, to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddHearthstone {
    pub(crate) name: HearthstoneName,
    pub(crate) template: HearthstoneTemplate,
}

impl AddHearthstone {
    /// Adds the Hearthstone alone with a manse and demense.
    pub fn with_manse(
        self,
        manse: impl Into<ManseName>,
    ) -> Result<ManseBuilderWithHearthstone, HearthstoneError> {
        ManseBuilder::name(manse).hearthstone(self)
    }

    /// Starts constructing a new Hearthstone with the given name.
    pub fn name(name: impl Into<HearthstoneName>) -> HearthstoneBuilder {
        HearthstoneBuilder {
            name: name.into(),
            book_reference: None,
        }
    }
}

impl From<AddHearthstone> for CharacterMutation {
    fn from(add_hearthstone: AddHearthstone) -> Self {
        AddMerit::from(add_hearthstone).into()
    }
}
