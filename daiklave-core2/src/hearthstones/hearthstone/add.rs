use crate::{merits::merit_new::{manse::{ManseName, builder::{ManseBuilderWithHearthstone, ManseBuilder}}}, hearthstones::HearthstoneError};

use super::{HearthstoneTemplate, HearthstoneName, builder::HearthstoneBuilder};

/// A hearthstone and its name, to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddHearthstone {
    name: HearthstoneName,
    pub(crate) template: HearthstoneTemplate,
}

impl AddHearthstone {
    pub fn with_manse(self, manse: impl Into<ManseName>) -> Result<ManseBuilderWithHearthstone, HearthstoneError> {
        ManseBuilder::name(manse).hearthstone(self)
    } 

    pub fn builder(name: impl Into<HearthstoneName>) -> HearthstoneBuilder {
        HearthstoneBuilder {
            name: name.into(),
            book_reference: None,
        }
    }
}