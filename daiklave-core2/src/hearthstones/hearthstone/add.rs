use super::{HearthstoneTemplate, HearthstoneName, builder::HearthstoneBuilder};

/// A hearthstone and its name, to be added to a character.
pub struct AddHearthstone {
    name: HearthstoneName,
    template: HearthstoneTemplate,
}

impl AddHearthstone {
    pub fn builder(name: impl ToString) -> HearthstoneBuilder {
        HearthstoneBuilder {
            name: name.into(),
            book_reference: None,
        }
    }
}