use super::HearthstoneTemplate;

/// A hearthstone and its name, to be added to a character.
pub type AddHearthstone = (String, HearthstoneTemplate);

/// Currently the only property of a manse is its name.
pub type Manse = String;

/// Currently the only propery of a demense is its name.
pub type Demense = String;

/// A manse, its demense, and its hearthstone, to be added to a character.
pub type AddManse = (Manse, Demense, AddHearthstone);
